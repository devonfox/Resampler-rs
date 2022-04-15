use hound::WavSpec;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Args: {}", env::args().count() - 1);

    let filename = env::args().nth(1).expect("no filename provided");
    let mut reader = hound::WavReader::open(&filename).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let oldspec: hound::WavSpec = reader.spec();

    let newspec = hound::WavSpec {
        channels: oldspec.channels,
        sample_rate: oldspec.sample_rate / 2,
        bits_per_sample: oldspec.bits_per_sample,
        sample_format: oldspec.sample_format,
    };

    let wav_samprate = oldspec.sample_rate;
    let duration = reader.duration() / oldspec.sample_rate;
    println!("Source File: '{}'", filename);
    println!("Duration: {} second(s)", duration);
    println!("Wav Sample Rate: {}", wav_samprate);
    resample(oldspec, newspec, samples, filename);
}

fn filter(mut samples: Vec<i16>) -> Vec<i16> {
    let file = File::open("coeffs.txt").expect("coeffs.txt not found");
    let parser = BufReader::new(file);
    let mut coeffs: Vec<f64> = Vec::new();
    for line in parser.lines() {
        let coeff: f64 = line
            .expect("error reading line")
            .trim()
            .parse::<f64>()
            .unwrap();
        // println!("{coeff}");
        coeffs.push(coeff);
    }
    // println!("Size of Coeffs vec: {}", coeffs.len());
    let ilength = samples.len();
    let jlength = coeffs.len();
    let mut filtered: Vec<i16> = Vec::new();
    for i in 92..ilength {
        let mut add: f64 = 0.0;
        for j in 0..jlength - 1 {
            let stuff = coeffs[j] * samples[i - j] as f64;
            add += stuff;
        }
        filtered.push(add as i16);
    }
    for i in 0..91 {
        filtered.insert(i, 0);
    }
    filtered
}

fn resample(oldspec: WavSpec, newspec: WavSpec, samples: Vec<i16>, filename: String) {
    let mut rfilename = filename;
    rfilename.insert(0, 'r');
    let mut rsamp_write = hound::WavWriter::create(&rfilename, newspec).unwrap();
    let count = samples.len() / 2;
    let resample = filter(samples);
    println!("\nInput sample rate: {}", oldspec.sample_rate);
    println!("Output sample rate: {}\n", newspec.sample_rate);
    for i in (0..resample.len()).step_by(2) {
        rsamp_write.write_sample(resample[i]).unwrap();
        // println!("{}", i);
    }

    println!("Created File: '{}'", rfilename);
    println!(
        "Duration: {} second(s)",
        (count as u32) / newspec.sample_rate
    );
    println!("Wav Sample Rate: {}\n", newspec.sample_rate);
    rsamp_write.finalize().unwrap();
}
