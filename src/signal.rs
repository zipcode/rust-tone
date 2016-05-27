use std::ops;
use std::iter;

#[derive(Clone)]
pub struct Signal {
    pub stream: Vec<i32>,
    pub precision: usize,
}

impl From<Vec<i32>> for Signal {
    fn from(v: Vec<i32>) -> Signal {
        Signal {
            stream: v,
            precision: 0,
        }
    }
}

impl Signal {
    #[allow(dead_code)]
    pub fn sum(&self) -> Signal {
        let mut vec: Vec<i32> = vec![];
        let mut sum = 0;
        for item in &self.stream {
            sum = sum + item;
            vec.push(sum);
        }
        Signal {
            stream: vec,
            precision: self.precision,
        }
    }

    #[allow(dead_code)]
    // This diff centers itself around each value so as to avoid adding noise if we
    // wish to relate a value to its undifferentiated version, eg s(t)/s'(t).
    pub fn diff(&self) -> Signal {
        let mut vec: Vec<i32> = vec![];
        let zero = 0;
        let mut last: Vec<&i32> = vec![&zero, &zero];
        for item in &self.stream {
            vec.push(item - last.remove(0));
            last.push(item);
        }
        vec.remove(0); // Throw away the first value
        vec.push(0 - last.remove(0)); // Pop on a final value
        Signal {
            stream: vec,
            precision: self.precision,
        }
    }

    #[allow(dead_code)]
    // This is obviously kinda flawed.
    // For starters it just assumes the other signal is a filter, and short.
    // If the filter is larger than 'self', we should transpose.
    // Also if the filter is > 64 in length we want to do an FFT convolution which is linear,
    // rather than this, which is entirely not.
    fn convolve(&self, other: &Signal) -> Signal {
        let mut filter: Vec<i32> = other.stream.clone();
        let unshift: usize = if self.precision >= other.precision { other.precision } else { self.precision };
        filter.reverse();
        let stream: Vec<i32> = iter::repeat(0).take(filter.len() - 1).chain(self.stream.clone()).collect();
        let result = stream.windows(filter.len()).map(|w| {
            w.iter().zip(filter.iter()).map(|(a, b)| (a * b) >> unshift).fold(0, |a, b| { a + b })
        }).collect();
        Signal {
            stream: result,
            precision: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.stream.len()
    }
}

impl ops::Add for Signal {
    type Output = Signal;
    fn add(self, rhs: Self::Output) -> Signal {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let shift = self.precision - rhs.precision;
        let res: Vec<i32> = self.stream.iter().zip(rhs.stream.iter()).map(|(a, b)| a + (b << shift)).collect();
        Signal {
            stream: res,
            precision: self.precision,
        }
    }
}

impl ops::Sub for Signal {
    type Output = Signal;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let shift = self.precision - rhs.precision;
        let res: Vec<i32> = self.stream.iter().zip(rhs.stream.iter()).map(|(a, b)| a - (b << shift)).collect();
        Signal {
            stream: res,
            precision: self.precision
        }
    }
}

impl ops::Mul for Signal {
    type Output = Signal;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let prec: usize = if self.precision < rhs.precision { rhs.precision } else { self.precision };
        let unshift: usize = if self.precision >= rhs.precision { rhs.precision } else { self.precision };
        let res: Vec<i32> = self.stream.iter()
          .zip(rhs.stream.iter())
          .map(|(a, b)| (a * b) >> unshift)
          .collect();
        Signal {
            stream: res,
            precision: prec,
        }
    }
}

impl ops::Div for Signal {
    type Output = Signal;
    fn div(self, rhs: Self::Output) -> Self::Output {
        assert!(self.stream.len() == rhs.stream.len(), "Stream length mismatch");
        let res: Vec<i32> = self.stream.iter()
            .zip(rhs.stream.iter())
            .map(|(a, b)| {
                if *b != 0 {
                    (*a << rhs.precision) / *b
                } else { 0 }
            })
            .collect();
        Signal {
            stream: res,
            precision: self.precision,
        }
    }
}

#[test]
fn test_sum() {
    let s = Signal::from(vec![0, 0, 1, 1, 1, 0]);
    let t = s.sum();
    assert!(t.stream == vec![0, 0, 1, 2, 3, 3]);
}

#[test]
fn test_sum_prec() {
    let s = Signal {
        stream: vec![0, 0, 1, 1, 1, 0].iter().map(|x| x << 6).collect(),
        precision: 6
    };
    let t = s.sum();
    let target: Vec<i32> = vec![0, 0, 1, 2, 3, 3].iter().map(|x| x << 6).collect();
    assert!(t.stream == target);
    assert!(t.precision == s.precision);
}

#[test]
fn test_diff() {
    let s = Signal::from(vec![9, 0, 1, 2, 3, 3]);
    let t = s.diff();
    println!("t.len:{} s.len:{}", t.stream.len(), s.stream.len());
    assert!(t.stream.len() == s.stream.len(), "Signal lengths should match");
    assert!(t.stream == vec![0, -8, 2, 2, 1, -3]);
}

#[test]
fn test_diff_prec() {
    let s = Signal {
        stream: vec![9 << 6, 0 << 6, 1 << 6, 2 << 6, 3 << 6, 3 << 6],
        precision: 6,
    };
    let t = s.diff();
    let target: Vec<i32> = vec![0, -8, 2, 2, 1, -3].iter().map(|x| x << 6).collect();
    println!("t: {:?}, target: {:?}", t.stream, target);
    assert!(t.stream == target);
    assert!(t.precision == s.precision);
}

#[test]
fn test_sum_diff() {
    // Explicitly enveloping this with 0s at either end.
    let s = Signal::from(vec![0, 1, 2, 3, 4, 3, 2, 1, 0, 9, 0]);
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
    let a = Signal::from(vec![1, 2, 3]);
    let b = Signal::from(vec![4, 5, 6]);
    let c = a + b;
    assert!(c.stream == vec![5, 7, 9]);
}

#[test]
fn test_sub() {
    let a = Signal::from(vec![4, 4, 4]);
    let b = Signal::from(vec![1, 2, 3]);
    let c = a - b;
    assert!(c.stream == vec![3, 2, 1]);
}

#[test]
fn test_mul() {
    let a = Signal::from(vec![0, 1, 2]);
    let b = Signal::from(vec![3, 3, 3]);
    let c = a * b;
    assert!(c.stream == vec![0, 3, 6]);
}

#[test]
fn test_mul_prec() {
    let a = Signal {
        stream: vec![0, 1 << 2, 2 << 2],
        precision: 2,
    };
    let b = Signal {
        stream: vec![3 << 1, 3 << 1, 3 << 1],
        precision: 1,
    };
    let c = a * b;
    assert!(c.stream == vec![0, 3 << 2, 6 << 2]);
}

#[test]
fn test_div() {
    let a = Signal::from(vec![4, 4, 4]);
    let b = Signal::from(vec![4, 2, 3]);
    let c: Signal = a / b;
    assert!(c.stream == vec![1, 2, 1]);
}

#[test]
fn test_div_prec() {
    let a = Signal {
        stream: vec![4 << 2, 4 << 2, 4 << 2, 100 << 2],
        precision: 2,
    };
    let b = Signal {
        stream: vec![4 << 1, 2 << 1, 3 << 1, 5 << 1],
        precision: 1,
    };
    let c = a / b;
    let expected: Vec<i32> = vec![1 << 2, 2 << 2, 5, 20 << 2];
    println!("\nExpected: {:?}\nGot:      {:?}", expected, c.stream);
    assert!(c.stream == expected);
    assert!(c.precision == 2);
}

#[test]
fn test_convolve() {
    let a = Signal {
        stream: vec![8, 9, 10, 4, 16],
        precision: 2,
    };
    let b = Signal {
        stream: vec![8],
        precision: 3,
    };
    let c = a.convolve(&b);
    let expected: Vec<i32> = a.stream.iter().map(|a| a << 1).collect();
    println!("\nExpected: {:?},\nGot:      {:?}", expected, c.stream);
    assert!(expected == c.stream);
}
