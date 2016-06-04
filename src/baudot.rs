#[allow(dead_code)]
fn decode(input_v: Vec<usize>) -> char {
    let v = input_v.clone();
    // Obviously we'll want a better lookup mechanism than this
    let baudot: Vec<(Vec<usize>, char)> = vec![
        (vec![0, 0, 0, 0, 0], '␀'), // null
        (vec![0, 1, 0, 0, 0], '␍'), // carriage return
        (vec![0, 0, 0, 1, 0], '␊'), // line feed
        (vec![0, 0, 1, 0, 0], ' '),
        (vec![1, 0, 1, 1, 1], 'Q'),
        (vec![1, 0, 0, 1, 1], 'W'),
        (vec![0, 0, 0, 0, 1], 'E'),
        (vec![0, 1, 0, 1, 0], 'R'),
        (vec![1, 0, 0, 0, 0], 'T'),
        (vec![1, 0, 1, 0, 1], 'Y'),
        (vec![0, 0, 1, 1, 1], 'U'),
        (vec![0, 0, 1, 1, 0], 'I'),
        (vec![1, 1, 0, 0, 0], 'O'),
        (vec![1, 0, 1, 1, 0], 'P'),
        (vec![0, 0, 0, 1, 1], 'A'),
        (vec![0, 0, 1, 0, 1], 'S'),
        (vec![0, 1, 0, 0, 1], 'D'),
        (vec![0, 1, 1, 0, 1], 'F'),
        (vec![1, 1, 0, 1, 0], 'G'),
        (vec![1, 0, 1, 0, 0], 'H'),
        (vec![0, 1, 0, 1, 1], 'J'),
        (vec![0, 1, 1, 1, 1], 'K'),
        (vec![1, 0, 0, 1, 0], 'L'),
        (vec![1, 0, 0, 0, 1], 'Z'),
        (vec![1, 1, 1, 0, 1], 'X'),
        (vec![0, 1, 1, 1, 0], 'C'),
        (vec![1, 1, 1, 1, 0], 'V'),
        (vec![1, 1, 0, 0, 1], 'B'),
        (vec![0, 1, 1, 0, 0], 'N'),
        (vec![1, 1, 1, 0, 0], 'M'),
        (vec![1, 1, 0, 1, 1], '↑'), // shift to figures
        (vec![1, 1, 1, 1, 1], '↓'), // shift to letters
    ];

    let baudot_rev: Vec<(Vec<usize>, char)> = baudot.iter().map(|x| {
        let (mut v, c) = x.clone();
        v.reverse();
        (v, c)
    }).collect();
    for (candidate, c) in baudot_rev {
        if candidate == v { return c }
    }
    println!("No match for {:?}", v);
    '?'
}

#[derive(Clone, Debug)]
enum DecodeState {
    Standby, // Waiting for start bit
    Reading(Vec<usize>),
    ExpectingStop(Vec<usize>),
}

pub fn decode_stream(s: Vec<usize>) -> String {
    let mut result: String = String::from("");
    let mut state: DecodeState = DecodeState::Standby;
    for i in (0..s.len()) {
        state = match state.clone() {
            DecodeState::Standby => {
                match s[i] {
                    0 => {
                        DecodeState::Reading(Vec::new())
                    },
                    _ => DecodeState::Standby,
                }
            },
            DecodeState::Reading(current) => {
                let mut my = current.clone();
                my.push(s[i]);
                match my.len() {
                    5 => DecodeState::ExpectingStop(my),
                    _ => DecodeState::Reading(my),
                }
            },
            DecodeState::ExpectingStop(current) => {
                match s[i] {
                    0 => DecodeState::Standby,
                    _ => {
                        let char = decode(current.clone());
                        result.push(char);
                        DecodeState::Standby
                    },
                }
            }
        };
    }
    result
}
