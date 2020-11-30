use crossbeam_channel;
use synth::wave;
use jack;

#[derive(PartialEq)]
enum ControlSignal {
    Record,
}

fn main() {
    let (client, status) =
      jack::Client::new("listener", jack::ClientOptions::NO_START_SERVER).unwrap();
    println!("{} : {:?}", client.name(), status);

    let ports = client.ports(None, None, jack::PortFlags::empty());
    for port in ports {
      println!("{}", port);
    }

    let audio_in = client
      .register_port("audio_in", jack::AudioIn::default())
      .unwrap();


    let (ctrl_tx, ctrl_rx) = crossbeam_channel::bounded::<ControlSignal>(0);
  
    let mut is_recording = false;
    let mut recording = Vec::new();
    let process = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            if Ok(ControlSignal::Record) == ctrl_rx.try_recv() {
                is_recording = !is_recording;
                if is_recording {
                    recording.clear();
                    println!("Recording...");
                } else {
                    wave::Wave{
                        samples: recording.clone(),
                    }.export("listener-out.wav");
                    println!("Recording exported!");
                }
            }
            if is_recording {
                recording.extend_from_slice(audio_in.as_slice(ps));
            }
            jack::Control::Continue
        }
    );

    let active_client = client.activate_async((), process).unwrap();

    // Standard input loop
    let mut buf = String::new();
    loop {
        std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "q" => break,
            "r" => { ctrl_tx.send(ControlSignal::Record).unwrap(); },
            _ => (),
        }
        buf.clear();
    }
    
    active_client.deactivate().unwrap(); 
}
