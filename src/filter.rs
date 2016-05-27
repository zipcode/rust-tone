use signal::Signal;
use std::f32::consts::PI;
use std::i32;

pub struct Kernel {
    sample_rate: f32,
}
pub struct Window;

impl Kernel {
    pub fn new(sample_rate: f32) -> Kernel {
        Kernel {
            sample_rate: sample_rate,
        }
    }

    pub fn unit(&self) -> Signal {
        Signal {
            stream: vec![1 << 16],
            precision: 16,
        }
    }

    pub fn sinc(&self, frequency: f32, bins: usize) -> Signal {
        let freq_fraction = frequency / self.sample_rate;
        let half: i32 = ((bins-1)/2) as i32;
        let real: Vec<f32> = (-half..half+1).map(|x| {
            let c = x as f32;
            if x == 0 {
                1.0
            } else {
                (2.0 * PI * c * freq_fraction).sin() / (c * PI)
            }
        }).collect();
        let scale: f32 = (1 << 15) as f32;
        let mapped: Vec<i32> = real.iter().map(|x| {
            ((*x as f32) * scale) as i32
        }).collect();
        Signal {
            stream: mapped,
            precision: 15,
        }
    }

    #[allow(dead_code)]
    pub fn windowed_sinc(&self, frequency: f32, bins: usize) -> Signal {
        self.sinc(frequency, bins) * Window::blackman(bins)
    }
}

impl Window {
    #[allow(dead_code)]
    pub fn blackman(bins: usize) -> Signal {
        let m = (bins - 1) as f32;
        let real: Vec<f32> = (0..bins).map(|x| {
            let c = x as f32;
            0.42 - 0.5 * (2.0 * PI * c / m).cos() + 0.08 * (4.0 * PI * c / m).cos()
        }).collect();
        let mapped: Vec<i32> = real.iter().map(|x| {
            (*x as f32 * (1 << 16) as f32) as i32
        }).collect();
        Signal {
            stream: mapped,
            precision: 16,
        }
    }
}
