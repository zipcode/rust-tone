extern crate hound;

mod nco;

use nco::NCOTable;

const DETECT: f32 = 1500.0;
const FUNDAMENTAL: f32 = 11025.0;

fn main() {
    let nco = NCOTable::new(FUNDAMENTAL, 16, 2);
    let mut osc = nco.freq(DETECT);
    for _ in (0 .. 20) {
        println!("V: {}", osc.next().unwrap());
    }
    osc.set_freq(DETECT * 0.25);
    for _ in (0 .. 20) {
        println!("V2: {}", osc.next().unwrap());
    }
}
