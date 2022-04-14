use std::f32::consts::PI;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let frequency = 440.0;
    let duration: u32 = spec.sample_rate; // one second
    output_basic_sine(spec, frequency, duration);
}

fn output_basic_sine(spec: hound::WavSpec, freq: f32, duration: u32) {
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..duration).map(|x| x as f32 / 48000.0) {
        let sample = (t * freq * 2.0 * PI).sin();
        let amplitude = (i16::MAX / 4) as f32; // one quarter max amplitude
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    println!("sine.wav created");
    writer.finalize().unwrap();
}
