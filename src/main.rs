use std::env;
use std::f32::consts::PI;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let filename = env::args().nth(1).expect("no filename provided");
    let srate = env::args().nth(2).expect("no sample rate provided");
    let srate = srate.trim().parse::<u32>().unwrap();
    let mut reader = hound::WavReader::open(&filename).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: srate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let frequency = 440.0;
    let duration: u32 = spec.sample_rate; // one second
    output_basic_sine(spec, frequency, duration);
    let newsamples = filter(samples);
    for sample in newsamples {
        println!("{:?}", sample);
    }
    
}

fn filter(mut samples: Vec<i16>) -> Vec<i16> {
    for sample in &mut samples {
        *sample /= 2;
    }
    samples
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
