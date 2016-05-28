extern crate hound;

mod filter;
mod nco;
mod signal;

use nco::NCOTable;
use signal::Signal;
use filter::Kernel;

const DETECT: f32 = 1417.0;
const FILE: &'static str = "RTTY_170Hz_45point45-01.wav";

fn main() {
    let rate: usize = 11025;
    let table = NCOTable::new(rate as f32, 16, 2);

    let filter: Signal = Kernel::new(rate as f32).windowed_sinc(700.0, 201);

    let s1 = table.sin(1000.0).into_signal(rate).scale(0.5);
    let s2 = table.sin(400.0).into_signal(rate).scale(0.5);
    let s3 = s1 + s2;

    let s4 = s3.convolve(&filter).convolve(&filter).convolve(&filter);

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
    for s in s4.stream {
        writer.write_sample((s * 32766.0) as i16).expect("Could not write sample");
    }
}
