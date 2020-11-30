mod scale;
mod voice;
mod wave;
use crossbeam_channel;
use jack;
use crate::scale::Scale;
use std::collections::HashMap;
use std::time::Duration;

#[derive(PartialEq)]
enum ControlSignal {
  PlayPause,
  NoteChange,
}

fn main() {
    let eq_temp = scale::EqualTemperament{
        ref_note: scale::Note {
           letter: scale::LetterNote::A,
           octave: 4,
        },
        ref_freq: 440.0,
        delta_ratio: 2f64.powf(1.0/12.0),
    };
    let c4_freq = eq_temp.get_frequency(scale::Note{
        letter: scale::LetterNote::C,
        octave: 4,
    });
    let c5_freq = eq_temp.get_frequency(scale::Note{
        letter: scale::LetterNote::C,
        octave: 5,
    });
    println!("Frequency: {}", c4_freq); 
    println!("Frequency: {}", c5_freq); 

    let (client, status) =
      jack::Client::new("tonegen", jack::ClientOptions::NO_START_SERVER).unwrap();
    println!("{} : {:?}", client.name(), status);

    let ports = client.ports(None, None, jack::PortFlags::empty());
    for port in ports {
      println!("{}", port);
    }

    let mut audio_out = client
      .register_port("audio_out", jack::AudioOut::default())
      .unwrap();

    let midi_in = client
      .register_port("midi_in", jack::MidiIn::default())
      .unwrap();

    // ?? : Make it so the Wave module can accept different sample rates? 
    //let mut wav = wave::Wave::tone(c4_freq, 0.8);
    let attack = Duration::new(0, 5_000_000);
    let decay = Duration::new(0, 500_000_000);
    let release = Duration::new(1, 0);
    let voice = voice::Voice::new(attack, decay, 0.8, release);

    let mut voice_releases = HashMap::new();
    let mut active_voices = Vec::new();

    let mut is_playing = true;
    let (ctrl_tx, ctrl_rx) = crossbeam_channel::bounded::<ControlSignal>(0);
    let (note_tx, note_rx) = crossbeam_channel::bounded::<scale::Note>(0);
    let process = jack::ClosureProcessHandler::new(
      move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {

        // Poll midi in
        for e in midi_in.iter(ps) {
          if e.bytes[0] == 144 {
            let note = scale::Note::from_midi_num(e.bytes[1].into()).unwrap();
            let freq = eq_temp.get_frequency(note);
            let pressure = e.bytes[2];
            if pressure > 0 {
              println!("Press! Note: {}, Freq: {}, MIDI num: {}, Pressure: {}", note, freq, e.bytes[1], pressure);
              let (conframe_rx, release_tx) = voice.press(freq, pressure as f64 / 127.0);
              // ?? : Replace with more intelligible keys
              voice_releases.insert(note, release_tx);
              active_voices.push((note, conframe_rx)); 
            } else {
              println!("Release! Note: {}", note);
              if let Some(release_tx) = voice_releases.get(&note) {
                release_tx.send(0).unwrap();
              }
            }
          } else {
            println!("Eyugh!");
          }
        }
        
        // Check for control signals
        if let Ok(sig) = ctrl_rx.try_recv() {
          // Can I do a match here?
          if sig == ControlSignal::PlayPause {
            is_playing = !is_playing;
          } else if sig == ControlSignal::NoteChange {
            if let Ok(note) = note_rx.recv() {
              println!("HAHA! {}", note.octave);
            }
          }
        }

        // Write to audio_out buffer
        if active_voices.len() > 0 {
          let (note, conframe_rx) = &mut active_voices[0];
          let out = audio_out.as_mut_slice(ps);
          for v in out.iter_mut() {
            if is_playing {
              // !! : Mix active voices
              *v = match conframe_rx.try_recv() {
                Ok(sample) => sample.0, // !!
                Err(crossbeam_channel::TryRecvError::Empty) => 0.0,
                Err(crossbeam_channel::TryRecvError::Disconnected) => {
                  voice_releases.remove(&note);
                  active_voices.remove(0);
                  break;
                }
              }
            } else {
              *v = 0.0;
            }
          }
        }
        jack::Control::Continue
      },
    );

    // Activate async client and connect I/O ports
    let active_client = client.activate_async((), process).unwrap();
    active_client.as_client().connect_ports_by_name("Nektar SE25 #0:midi.TX", "tonegen:midi_in").unwrap();
    active_client.as_client().connect_ports_by_name("tonegen:audio_out", "system:playback_1").unwrap();
    active_client.as_client().connect_ports_by_name("tonegen:audio_out", "system:playback_2").unwrap();
    active_client.as_client().connect_ports_by_name("tonegen:audio_out", "listener:audio_in").unwrap();

    // Standard input loop
    let mut buf = String::new();
    loop {
      std::io::stdin().read_line(&mut buf).unwrap();
      match buf.trim() {
        "q" => break,
        "p" => { ctrl_tx.send(ControlSignal::PlayPause).unwrap(); },
        _ => {
            ctrl_tx.send(ControlSignal::NoteChange).unwrap();
            note_tx.send(scale::Note::from_str(buf.trim()).unwrap()).unwrap();
          }
      }
      buf.clear();
    }

    active_client.deactivate().unwrap(); 
}
