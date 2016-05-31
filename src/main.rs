extern crate hound;

mod filter;
mod nco;
mod signal;

use nco::NCOTable;
use signal::Signal;
use filter::Kernel;

const DETECT: f32 = 1500.0;
const FILE: &'static str = "RTTY_170Hz_45point45-01.wav";

fn main() {
    let mut reader = hound::WavReader::open(FILE).expect("file could not be loaded");
    let samples: Vec<f32> = reader.samples::<i16>().map(|s| {
        s.expect("could not read sample") as f32 / 32766.0
    }).collect();
    let r: Signal = Signal::from(samples);
    let rate = reader.spec().sample_rate;

    let table = NCOTable::new(rate as f32, 16, 2);
    let filterp: Signal = Kernel::new(rate as f32).windowed_sinc(700.0, 101);
    let filter: Signal = filterp.convolve(&filterp);

    let size = r.len();
    let i: Signal = (table.sin(DETECT).into_signal(size) * r.clone()).filter(&filter);
    let q: Signal = (table.cos(DETECT).into_signal(size) * r.clone()).filter(&filter);
    let id: Signal = i.clone().diff();
    let qd: Signal = q.clone().diff();
    let i2: Signal = i.clone() * i.clone();
    let q2: Signal = q.clone() * q.clone();

    let result = ((i * qd - q * id) / (i2 + q2)).filter(&filter);

    let bits = result.into_bitstream(0.05);
    println!("{:?}", bits);
}
