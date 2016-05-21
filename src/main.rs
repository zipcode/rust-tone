extern crate hound;

mod nco;

use nco::NCOTable;
use std::i16;

const DETECT: f32 = 1500.0;
const FUNDAMENTAL: f32 = 11025.0;

fn main() {
    let nco = NCOTable::new(FUNDAMENTAL, 16, 2);
    let mut osc = nco.freq(DETECT - (170.0/2.0));
    osc.differentiate();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: FUNDAMENTAL as u32,
        bits_per_sample: 16,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for _ in (0 .. 20) {
        let amp = i16::MAX as f32;
        let sample = osc.next().unwrap();
        writer.write_sample((sample * amp) as i16).unwrap();
    }
}
