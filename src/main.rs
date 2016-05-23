extern crate hound;

mod nco;
mod signal;

use nco::NCOTable;

const DETECT: f32 = 1500.0;
const FUNDAMENTAL: f32 = 11025.0;

fn main() {
    let nco = NCOTable::new(FUNDAMENTAL, 16, 2);
    let osc = nco.freq(DETECT - (170.0/2.0));

}
