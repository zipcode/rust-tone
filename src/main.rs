extern crate hound;

use std::i16;

mod signal;
use signal::Signal;

const DETECT: f32 = 1500.0;

fn main() {
    // Mark/space around 1410Hz and 1587Hz, nominally 170Hz apart
    // Maybe go for 1500Hz as the center?
    // 1575Hz would be an even divisor of the sample rate, mind.
    let sample = "RTTY_170Hz_45point45-01.wav";
    let detect = DETECT;
    println!("Loading from {}", sample);
    let reader = match hound::WavReader::open(sample) {
        Ok(stream) => stream,
        Err(_) => {
            println!("FATAL: Could not read {}", sample);
            return
        }
    };

    // Ensure the file we're reading conforms to the assumptions we made
    assert!(reader.spec().channels == 1);
    assert!(reader.spec().bits_per_sample == 16);

    let rate = reader.spec().sample_rate;

    let stream = reader.into_samples::<i16>().into_iter().map(|x|
        x.expect("Read error") as f32 / i16::MAX as f32
    );

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: rate,
        bits_per_sample: 16
    };
    let mut writer = match hound::WavWriter::create("sine.wav", spec) {
        Ok(writer) => writer,
        Err(_) => {
            println!("FATAL: Could not open sine.wav for writing");
            return
        }
    };

    let oscillator = Signal::sine(rate, detect);
    let amplitude = i16::MAX as f32;
    for s in oscillator.iter().take(rate as usize) {
        writer.write_sample((s * amplitude) as i16);
    };
}
