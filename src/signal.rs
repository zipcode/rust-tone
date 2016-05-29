use std::ops;
use std::iter;

#[derive(Clone, Debug)]
pub struct Signal {
    pub stream: Vec<f32>,
}

#[derive(Clone,Debug)]
pub struct BitStream {
    pub stream: Vec<usize>,
}

impl From<Vec<f32>> for Signal {
    fn from(v: Vec<f32>) -> Signal {
        Signal {
            stream: v,
        }
    }
}

impl Signal {
    #[allow(dead_code)]
    pub fn sum(&self) -> Signal {
        let mut vec: Vec<f32> = vec![];
        let mut sum = 0.0;
        for item in &self.stream {
            sum = sum + item;
            vec.push(sum);
        }
        Signal {
            stream: vec,
        }
    }

    #[allow(dead_code)]
    // This diff centers itself around each value so as to avoid adding noise if we
    // wish to relate a value to its undifferentiated version, eg s(t)/s'(t).
    pub fn diff(&self) -> Signal {
        let mut vec: Vec<f32> = vec![];
        let zero = 0.0;
        let mut last: Vec<&f32> = vec![&zero, &zero];
        for item in &self.stream {
            vec.push(item - last.remove(0));
            last.push(item);
        }
        vec.remove(0); // Throw away the first value
        vec.push(0.0 - last.remove(0)); // Pop on a final value
        Signal {
            stream: vec,
        }
    }

    #[allow(dead_code)]
    // This is obviously kinda flawed.
    // For starters it just assumes the other signal is a filter, and short.
    // If the filter is larger than 'self', we should transpose.
    // Also if the filter is > 64 in length we want to do an FFT convolution which is linear,
    // rather than this, which is entirely not.
    pub fn convolve(&self, other: &Signal) -> Signal {
        let mut filter: Vec<f32> = other.stream.clone();
        filter.reverse();
        let stream: Vec<f32> = iter::repeat(0.0).take(filter.len() - 1).chain(self.stream.clone()).collect();
        let result = stream.windows(filter.len()).map(|w| {
            w.iter().zip(filter.iter()).map(|(a, b)| *a * *b).fold(0.0, |a, b| { a + b })
        }).collect();
        Signal {
            stream: result,
        }
    }

    pub fn filter(&self, other: &Signal) -> Signal {
        let mut s = self.convolve(&other);
        let len = other.len() * 2;
        for _ in (0..len) { s.stream.remove(0); }
        s
    }

    #[allow(dead_code)]
    pub fn scale(self, value: f32) -> Signal {
        let res: Vec<f32> = self.stream.iter().map(|x| {
            x * value
        }).collect();
        Signal {
            stream: res,
        }
    }

    pub fn len(&self) -> usize {
        self.stream.len()
    }

    #[allow(dead_code)]
    pub fn into_bitstream(self, threshold: f32) -> BitStream {
        let mut res: Vec<usize> = vec![];
        let mut state: usize = 0;
        for s in self.stream {
            let t: f32 = if state == 0 { threshold } else { -threshold };
            if s >= t {
                state = 1;
                res.push(1);
            } else {
                state = 0;
                res.push(0);
            }
        }
        BitStream {
            stream: res,
        }
    }
}

impl ops::Add for Signal {
    type Output = Signal;
    fn add(self, rhs: Self::Output) -> Signal {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let res: Vec<f32> = self.stream.iter().zip(rhs.stream.iter()).map(|(a, b)| a + b).collect();
        Signal {
            stream: res,
        }
    }
}

impl ops::Sub for Signal {
    type Output = Signal;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let res: Vec<f32> = self.stream.iter().zip(rhs.stream.iter()).map(|(a, b)| a - b).collect();
        Signal {
            stream: res,
        }
    }
}

impl ops::Mul for Signal {
    type Output = Signal;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let res: Vec<f32> = self.stream.iter()
            .zip(rhs.stream.iter())
            .map(|(a, b)| {
                *a * *b
            })
            .collect();
        Signal {
            stream: res,
        }
    }
}

impl ops::Div for Signal {
    type Output = Signal;
    fn div(self, rhs: Self::Output) -> Self::Output {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let res: Vec<f32> = self.stream.iter()
            .zip(rhs.stream.iter())
            .map(|(a, b)| {
                *a / *b
            })
            .collect();
        Signal {
            stream: res,
        }
    }
}

#[test]
fn test_sum() {
    let s = Signal::from(vec![0.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
    let t = s.sum();
    assert!(t.stream == vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0]);
}

#[test]
fn test_diff() {
    let s = Signal::from(vec![9.0, 0.0, 1.0, 2.0, 3.0, 3.0]);
    let t = s.diff();
    println!("t.len:{} s.len:{}", t.stream.len(), s.stream.len());
    assert!(t.stream.len() == s.stream.len(), "Signal lengths should match");
    assert!(t.stream == vec![0.0, -8.0, 2.0, 2.0, 1.0, -3.0]);
}

#[test]
fn test_sum_diff() {
    // Explicitly enveloping this with 0s at either end.
    let s = Signal::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0, 0.0, 9.0, 0.0]);
    let mut t = s.diff().sum();
    let mut u = s.sum().diff();
    println!("\ns:  {:?}\n+:  {:?}\n/:  {:?}\n\n/+: {:?}\n+-: {:?}", s.stream, s.sum().stream, s.diff().stream, t.stream, u.stream);
    // That last value is gonna be garbage
    u.stream.pop();
    t.stream.pop();
    assert!(u.stream == t.stream, "diff/sum should be vaguely symmetric");
}

#[test]
fn test_add() {
    let a = Signal::from(vec![1.0, 2.0, 3.0]);
    let b = Signal::from(vec![4.0, 5.0, 6.0]);
    let c = a + b;
    assert!(c.stream == vec![5.0, 7.0, 9.0]);
}

#[test]
fn test_sub() {
    let a = Signal::from(vec![4.0, 4.0, 4.0]);
    let b = Signal::from(vec![1.0, 2.0, 3.0]);
    let c = a - b;
    assert!(c.stream == vec![3.0, 2.0, 1.0]);
}

#[test]
fn test_mul() {
    let a = Signal::from(vec![0.0, 1.0, 2.0]);
    let b = Signal::from(vec![3.0, 3.0, 3.0]);
    let c = a * b;
    assert!(c.stream == vec![0.0, 3.0, 6.0]);
}

#[test]
fn test_div() {
    let a = Signal::from(vec![4.0, 4.0, 4.0]);
    let b = Signal::from(vec![4.0, 2.0, 3.0]);
    let c: Signal = a / b;
    let r: Vec<f32> = c.stream.iter().map(|x| x.round()).collect();
    assert!(r == vec![1.0, 2.0, 1.0]);
}

#[test]
fn test_convolve() {
    let a = Signal {
        stream: vec![8.0, 9.0, 10.0, 4.0, 16.0],
    };
    let b = Signal {
        stream: vec![1.0],
    };
    let c = a.convolve(&b);
    println!("\nExpected: {:?},\nGot:      {:?}", a.stream, c.stream);
    assert!(a.stream == c.stream);
}

#[test]
fn test_into_bitstream() {
    let s: Signal = Signal {
        stream: vec![0.0, 0.0, 0.8, 0.8, -0.1, 0.8, -0.6, -0.5, -0.8],
    };
    let r = s.into_bitstream(0.5);
    assert!(r.stream == vec![0, 0, 1, 1, 1, 1, 0, 0, 0]);
}
