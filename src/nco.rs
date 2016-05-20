use std::f32::consts::PI;

pub struct NCO {
    pub bits: usize,
    pub fractional: usize,
    pub fundamental: f32,
    samples: Vec<f32>,
}

impl NCO {
    // fundamental - the sample rate, effectively. eg 44100Hz.
    // bits - resolution of the lookup table
    // fractional - allow for tinier steps
    // In short this works by using integer fractions of the fundamental
    // eg if your fundamental is 44100Hz and your bits are 8, you can
    // only oscillate at 1, 1/2, 1/3, 1/4 of the (1<<8) entries.
    // fractional resolution lets you get closer to other values.
    #[allow(dead_code)]
    pub fn new(fundamental: f32, bits: usize, fractional: usize) -> NCO {
        let max = 1 << bits;
        NCO {
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

impl<'a> NCOStep<'a> for NCO {
    fn step(&'a self, step: usize) -> NCOIterator<'a> {
        NCOIterator {
            index: 0,
            step: step,
            nco: &self,
        }
    }

    fn freq(&'a self, freq: f32) -> NCOIterator<'a> {
        let step = self.steps_for_freq(freq);
        NCOIterator {
            index: 0,
            step: step,
            nco: &self,
        }
    }
}

struct NCOIterator<'a> {
    index: usize,
    pub step: usize,
    pub nco: &'a NCO,
}

impl<'a> NCOStep<'a> for NCOIterator<'a> {
    fn step(&self, step: usize) -> NCOIterator<'a> {
        NCOIterator {
            index: self.index,
            step: step,
            nco: self.nco,
        }
    }

    fn freq(&self, freq: f32) -> NCOIterator<'a> {
        let step = self.nco.steps_for_freq(freq).clone();
        NCOIterator {
            index: 0,
            step: step,
            nco: self.nco,
        }
    }
}

pub trait NCOStep<'a> {
    fn step(&'a self, step:usize) -> NCOIterator<'a>;
    fn freq(&'a self, freq:f32) -> NCOIterator<'a>;
}

impl<'a> Iterator for NCOIterator<'a> {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let result = self.nco.samples[self.index >> self.nco.fractional];
        let max = 1 << (self.nco.bits + self.nco.fractional);
        self.index = (self.index + self.step) % max;
        Some(result)
    }
}
