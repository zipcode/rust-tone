use std::f32::consts::PI;

pub struct NCOTable {
    pub bits: usize,
    pub fractional: usize,
    pub fundamental: f32,
    samples: Vec<f32>,
}

impl NCOTable {
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
}

impl<'a> NCOStep<'a> for NCOTable {
    fn step(&'a self, step: usize) -> NCO<'a> {
        NCO {
            index: 0,
            step: step,
            table: &self,
        }
    }

    fn freq(&'a self, freq: f32) -> NCO<'a> {
        let step = self.steps_for_freq(freq);
        NCO {
            index: 0,
            step: step,
            table: &self,
        }
    }
}

struct NCO<'a> {
    index: usize,
    pub step: usize,
    table: &'a NCOTable,
}

impl<'a> NCOStep<'a> for NCO<'a> {
    fn step(&self, step: usize) -> NCO<'a> {
        NCO {
            index: self.index,
            step: step,
            table: self.table,
        }
    }

    fn freq(&self, freq: f32) -> NCO<'a> {
        let step = self.table.steps_for_freq(freq).clone();
        NCO {
            index: 0,
            step: step,
            table: self.table,
        }
    }
}

pub trait NCOStep<'a> {
    fn step(&'a self, step:usize) -> NCO<'a>;
    fn freq(&'a self, freq:f32) -> NCO<'a>;
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