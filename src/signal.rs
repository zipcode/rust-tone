use std::f32::consts::PI;

#[allow(dead_code)]
pub struct Signal {
    rate: u32,
    samples: Vec<f32>,
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

impl Signal {
    #[allow(dead_code)]
    pub fn sine(rate: u32, frequency: f32) -> Signal {
        assert!(frequency * 2.0 < rate as f32, "Failed Nyquist criterion");
        let count = (rate as f32 / gcd(rate, frequency as u32) as f32) as u32;
        let samples = (0..count).map(|step| (step as f32) / (rate as f32)).map(|t| {
            (t * 2.0 * PI * frequency).sin()
        }).collect();
        Signal {
            rate: rate,
            samples: samples,
        }
    }

    #[allow(dead_code)]
    pub fn iter(self) -> SignalIterator {
        SignalIterator {
            index: 0,
            signal: self,
        }
    }
}

struct SignalIterator {
    index: usize,
    signal: Signal,
}

impl Iterator for SignalIterator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let result = self.signal.samples[self.index];
        self.index = (self.index + 1) % self.signal.samples.len();
        Some(result)
    }
}
