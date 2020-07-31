mod wave;
use jack;

fn main() {
    let (client, status) =
      jack::Client::new("tone-gen", jack::ClientOptions::NO_START_SERVER).unwrap();
    println!("{} : {:?}", client.name(), status);

    let mut port = client
      .register_port("tone-out", jack::AudioOut::default())
      .unwrap();
 
    let freq = 2000.0;
    let sample_rate = client.sample_rate() as u32;
    let w = wave::Wave::tone(freq, 0.8, 0.8, sample_rate);
    //let w = wave::Wave::silence(3.0, 44_100);
    println!("{:?}", w.samples.len());
    let export_path = format!("{}.wav", freq.to_string());
    w.export(&export_path);
}
