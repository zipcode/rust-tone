extern crate hound;

mod nco;

use nco::NCO;
use nco::NCOStep;

const DETECT: f32 = 1500.0;
const FUNDAMENTAL: f32 = 11025.0;

fn main() {
    let nco = NCO::new(FUNDAMENTAL, 16, 2);
    let mut osc = nco.freq(DETECT);
    let steps = osc.step;
    let size = 1 << (nco.bits + nco.fractional);
    println!("S: {} of {}", steps, size);
    for i in (0 .. 20) {
        println!("V: {}", osc.next().unwrap());
    }
    let mut osc2 = osc.freq(DETECT * 0.25);
    println!("S: {} of {}", osc2.step, size);
    for i in (0 .. 20) {
        println!("V2: {}", osc2.next().unwrap());
    }
}
