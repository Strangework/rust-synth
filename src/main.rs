mod wave;

fn main() {
    let freq = 2000.0;
    let w = wave::Wave::tone(freq, 0.8, 1.0, 44_100);
    //let w = wave::Wave::silence(3.0, 44_100);
    println!("{:?}", w.samples.len());
    let export_path = format!("{}.wav", freq.to_string());
    w.export(&export_path);
}
