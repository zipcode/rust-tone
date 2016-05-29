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

    let spec = hound::WavSpec {
        channels: 1,
        bits_per_sample: 16,
        sample_rate: rate as u32,
    };
    let mut writer = match hound::WavWriter::create("sine.wav", spec) {
        Ok(w) => w,
        Err(_) => {
            println!("Could not open output for writing");
            return
        }
    };
    let scale: f32 = (1 << 15) as f32 - 2.0; // Negative values only go down to this!
    for x in result.stream {
        let s = (if x.abs() > 1.0 { 1.0 * x.signum() } else { x }) * scale;
        writer.write_sample(s as i16).expect("Could not write sample");
    };
}
