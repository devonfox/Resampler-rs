use hound::WavSpec;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // Takes first argument as a filename to a wav file to resample to half the rate
    let filename = env::args().nth(1).expect("no filename provided");
    // creates a wav reader from 'hound' crate
    let mut reader = hound::WavReader::open(&filename).unwrap();

    // maps samples and collects to a vec, unwrapping result in the process
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let inspec: hound::WavSpec = reader.spec();
    assert_eq!(inspec.channels, 1, "mono input files only.");

    let outspec = hound::WavSpec {
        channels: inspec.channels,
        sample_rate: inspec.sample_rate / 2,
        bits_per_sample: inspec.bits_per_sample,
        sample_format: inspec.sample_format,
    };

    let wav_samprate = inspec.sample_rate;
    let duration = reader.duration() / inspec.sample_rate;
    println!("Source File: '{}'", filename);
    println!("Duration: {} second(s)", duration);
    println!("Wav Sample Rate: {} sps", wav_samprate);
    resample(inspec, outspec, samples, filename);
}

/// FIR filter function utilizes an external file of filter coefficients
/// provided by Bart Massey
fn filter(samples: Vec<i16>) -> Vec<i16> {
    // Read from coeffs.txt provided by Bart Massey
    let file = File::open("coeffs.txt").expect("coeffs.txt not found");
    let parser = BufReader::new(file);
    let mut coeffs: Vec<f64> = Vec::new();
    for line in parser.lines() {
        let coeff: f64 = line
            .expect("error reading line")
            .trim()
            .parse::<f64>()
            .unwrap();
        coeffs.push(coeff);
    }

    let ilength = samples.len();
    let jlength = coeffs.len();
    let mut filtered: Vec<i16> = Vec::new();
    for i in jlength..ilength {
        let mut add: f64 = 0.0;
        for j in 0..jlength - 1 {
            let sampsum = coeffs[j] * samples[i - j] as f64;
            add += sampsum;
        }
        filtered.push(add as i16);
    }
    for i in 0..jlength {
        filtered.insert(i, 0);
    }
    filtered
}

/// Resample function takes vector of samples, calls FIR filter function on 'samples'
/// and outputs every other sample to a wave file
fn resample(inspec: WavSpec, outspec: WavSpec, samples: Vec<i16>, filename: String) {
    let mut rfilename = filename;
    rfilename.insert(0, 'r');
    let mut rsamp_write = hound::WavWriter::create(&rfilename, outspec).unwrap();
    let count = samples.len() / 2;
    let resample = filter(samples);

    println!("\nInput sample rate: {} sps", inspec.sample_rate);
    println!("Output sample rate: {} sps\n", outspec.sample_rate);
    for i in (0..resample.len()).step_by(2) {
        rsamp_write.write_sample(resample[i]).unwrap();
    }

    println!("Created File: '{}'", rfilename);
    println!(
        "Duration: {} second(s)",
        (count as u32) / outspec.sample_rate
    );
    println!("Wav Sample Rate: {} sps\n", outspec.sample_rate);
    rsamp_write.finalize().unwrap();
}
