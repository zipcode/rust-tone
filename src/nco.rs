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
        let phase_step = ((phase / (2.0 * PI)) * self.step as f32) as usize % self.step;
        self.index = phase_step;
    }

    #[allow(dead_code)]
    pub fn shift_phase(&mut self, phase: f32) {
        let phase_step = ((phase / (2.0 * PI)) * self.step as f32) as usize % self.step;
        let max = 1 << (self.table.bits + self.table.fractional);
        self.index = (self.index + phase_step) % max;
    }

    #[allow(dead_code)]
    pub fn differentiate(&mut self) {
        self.shift_phase(PI*0.5);
    }

    #[allow(dead_code)]
    pub fn integrate(&mut self) {
        self.shift_phase(PI*-0.5);
    }
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

impl<'a> Clone for NCO<'a> {
    fn clone(&self) -> NCO<'a> {
        NCO {
            index: self.index,
            step: self.step,
            table: self.table,
        }
    }
}
