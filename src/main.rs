extern crate hound;

mod nco;
mod signal;

use nco::NCOTable;
use signal::Signal;

const DETECT: f32 = 1500.0;
const FILE: &'static str = "RTTY_170Hz_45point45-01.wav";
const SKIP: usize = 2000;

fn main() {
    let reader = match hound::WavReader::open(FILE) {
        Ok(stream) => stream,
        Err(_) => {
            println!("FATAL: could not read {}", FILE);
            return;
        }
    };
    let rate = reader.spec().sample_rate;
    let table = NCOTable::new(rate as f32, 16, 8);

    let signal = Signal {
        stream: reader.into_samples::<i16>().into_iter().map(|x| {
            x.expect("Read error") as i32
        }).collect(),
        precision: 16,
    };

    let i = table.sin(DETECT).into_signal(signal.len()) * signal.clone();
    let q = table.cos(DETECT).into_signal(signal.len()) * signal.clone();
    let idiff = i.diff();
    let qdiff = q.diff();
    let i2 = i.clone() * i.clone();
    let q2 = q.clone() * q.clone();
    let denominator = i2.clone() + q2.clone();
    let numerator = (i.clone() * qdiff.clone()) - (q.clone() * idiff.clone());
    let result = numerator.clone() / denominator.clone();

    println!("I  {:?}", i.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());
    println!("Q  {:?}", q.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());
    println!("Id {:?}", idiff.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());
    println!("Qd {:?}", qdiff.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());

    println!("I2 {:?}", i2.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());
    println!("Q2 {:?}", q2.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());

    println!("N  {:?}", numerator.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());
    println!("D  {:?}", denominator.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());

    println!("R  {:?}", result.clone().stream.iter().skip(SKIP).take(10).map(|x| x.clone()).collect::<Vec<i32>>());

    println!("I {}  Q {}  Id {}  Qd {}  I2 {}  Q2 {}  N {}  D {}  R {}",
        i.precision, q.precision, idiff.precision, qdiff.precision, i2.precision, q2.precision, numerator.precision, denominator.precision, result.precision);

    let spec = hound::WavSpec {
        channels: 1,
        bits_per_sample: 16,
        sample_rate: rate,
    };
    let mut writer = match hound::WavWriter::create("sine.wav", spec) {
        Ok(w) => w,
        Err(_) => {
            println!("Could not open output for writing");
            return
        }
    };
    for s in numerator.stream {
        writer.write_sample(s as i16).expect("Could not write sample");
    }
}
