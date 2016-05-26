use std::f32::consts::PI;

// An initial oscillator table
// This should be long-lived: it's doing all the computation up front.
pub struct NCOTable {
    pub bits: usize,
    pub fractional: usize,
    pub fundamental: f32,
    samples: Vec<f32>,
}

// Here's your numerically-controlled oscillator.
#[derive(Clone)]
struct NCO<'a> {
    pub index: usize,
    pub step: usize,
    table: &'a NCOTable,
}

impl<'a> NCOTable {
    // fundamental - the sample rate, effectively. eg 44100Hz.
    // bits - resolution of the lookup table
    // fractional - allow for tinier steps
    // In short this works by using integer fractions of the fundamental
    // eg if your fundamental is 44100Hz and your bits are 8, you can
    // only oscillate at 1, 1/2, 1/3, 1/4 of the (1<<8) entries.
    // fractional resolution lets you get closer to other values.
    #[allow(dead_code)]
    pub fn new(fundamental: f32, bits: usize, fractional: usize) -> NCOTable {
        let max = 1 << bits;
        NCOTable {
            bits: bits,
            fractional: fractional,
            fundamental: fundamental,
            samples: (0 .. max).map(|s| {
                ((s as f32 / max as f32) * 2.0 * PI).sin()
            }).collect(),
        }
    }

    pub fn steps_for_freq(&self, freq: f32) -> usize {
        ((1 << (self.bits + self.fractional)) as f32 * (freq / self.fundamental)).floor() as usize
    }

    #[allow(dead_code)]
    pub fn step(&'a self, step: usize) -> NCO<'a> {
        NCO {
            index: 0,
            step: step,
            table: &self,
        }
    }

    #[allow(dead_code)]
    pub fn freq(&'a self, freq: f32) -> NCO<'a> {
        let step = self.steps_for_freq(freq);
        NCO {
            index: 0,
            step: step,
            table: &self,
        }
    }

    pub fn sin(&'a self, freq: f32) -> NCO<'a> {
        Self::freq(self, freq)
    }

    pub fn cos(&'a self, freq: f32) -> NCO<'a> {
        let mut osc = Self::sin(self, freq);
        osc.set_phase(PI/2.0);
        osc
    }
}

#[test]
fn test_sin_cos() {
    let table = NCOTable::new(44100.0, 8, 4);
    let mut s = table.sin(500.0);
    let mut c = table.cos(500.0);
    let cval = c.next().unwrap();
    println!("{}", cval);
    assert!(s.next() == Some(0.0), "Sin should be 0.0");
    assert!(cval > 0.99, "Cos should be 1.0");
}

impl<'a> NCO<'a> {
    #[allow(dead_code)]
    pub fn set_step(&mut self, step: usize) {
        self.step = step;
    }

    #[allow(dead_code)]
    pub fn set_freq(&mut self, freq: f32) {
        self.set_step(self.table.steps_for_freq(freq).clone());
    }

    #[allow(dead_code)]
    pub fn set_phase(&mut self, phase: f32) {
        let max = 1 << (self.table.bits + self.table.fractional);
        let phase_step = ((phase / (2.0 * PI)) * max as f32) as usize % max;
        self.index = phase_step;
    }

    #[allow(dead_code)]
    pub fn shift_phase(&mut self, phase: f32) {
        let max = 1 << (self.table.bits + self.table.fractional);
        let phase_step = ((phase / (2.0 * PI)) * max as f32) as usize % max;
        self.index = (self.index + phase_step) % max;
    }
}

#[test]
fn test_set_phase() {
    let table = NCOTable::new(44100.0, 8, 2);
    let mut osc = table.freq(500.0);
    assert!(osc.index == 0, "Initial index should be 0");
    osc.set_phase(PI/2.0);
    println!("oscillator index is {}", osc.index);
    // 1/4 of the way around is 1 << 8, since 1 << (8 + 2) is the full table size
    assert!(osc.index == 256, "Phase-shifted index should be 1/4 around the values");
}

impl<'a> Iterator for NCO<'a> {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let result = self.table.samples[self.index >> self.table.fractional];
        let max = 1 << (self.table.bits + self.table.fractional);
        self.index = (self.index + self.step) % max;
        Some(result)
    }
}
