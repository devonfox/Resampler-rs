use std::env;
use std::f32::consts::PI;

use hound::WavSpec;

fn main() {
    println!("Args: {}", env::args().count() - 1);

    let filename = env::args().nth(1).expect("no filename provided");
    let mut reader = hound::WavReader::open(&filename).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    // let srate = env::args().nth(2).expect("no sample rate provided");
    // let srate = srate.trim().parse::<u32>().unwrap();
    let oldspec: hound::WavSpec = reader.spec();

    let newspec = hound::WavSpec {
        channels: oldspec.channels,
        sample_rate: oldspec.sample_rate / 2,
        bits_per_sample: oldspec.bits_per_sample,
        sample_format: oldspec.sample_format,
    };

    let wav_samprate = oldspec.sample_rate;
    println!("Wav Sample Rate: {}", wav_samprate);
    resample(oldspec, newspec, samples, filename);
}

fn filter(mut samples: Vec<i16>) -> Vec<i16> {
    for sample in &mut samples {
        *sample /= 2;
    }
    samples
}

fn resample(oldspec: WavSpec, newspec: WavSpec, samples: Vec<i16>, filename: String) {
    let mut rfilename = String::new();
    rfilename = filename;
    rfilename.insert(0, 'r');
    let mut rsamp_write = hound::WavWriter::create(rfilename, newspec).unwrap();
    let resample = filter(samples);
    println!("Input sample rate: {}", oldspec.sample_rate);
    println!("Output sample rate: {}", newspec.sample_rate);
    for sample in resample {
        rsamp_write.write_sample(sample).unwrap();
    }
    
    rsamp_write.finalize().unwrap();
}
