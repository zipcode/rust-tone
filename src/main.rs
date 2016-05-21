extern crate hound;

mod nco;

use std::f32::consts::PI;
use nco::NCOTable;

const DETECT: f32 = 1500.0;
const FUNDAMENTAL: f32 = 11025.0;

fn main() {
    let nco = NCOTable::new(FUNDAMENTAL, 16, 2);
    let mut osc = nco.freq(DETECT);
    println!("Index: {}", osc.index);
    osc.shift_phase(-1.0*PI);
    println!("Index: {}", osc.index);
    osc.set_phase(3.0*PI);
    println!("Index: {}", osc.index);
}
