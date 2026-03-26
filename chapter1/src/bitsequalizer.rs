use std::{
    cmp::Ordering,
    io::{self, Read},
    mem,
};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut input = Vec::with_capacity(lines.next().unwrap().parse::<usize>().unwrap());
    let mut accum = ["nil"; 2];
    for (i, l) in lines.enumerate() {
        accum[i % 2] = l;
        if i != 0 && i % 2 != 0 {
            input.push(accum.map(ToString::to_string));
        }
    }
    for (i, [mut a, b]) in input.into_iter().enumerate() {
        let (mut available, mut required) = (Vec::new(), Vec::new());
        for (i, (input_by, target_by)) in a.bytes().zip(b.bytes()).enumerate() {
            if matches!((input_by, target_by), (b'1', b'0')) {
                available.push(i);
            } else if matches!(input_by, b'0' | b'?') && target_by != b'0' {
                required.push((i, input_by));
            }
        }
        required.sort_unstable_by(|(_, by1), (_, by2)| match (by1, by2) {
            | (b'0', b'0') | (b'?', b'?') => Ordering::Equal,
            | (b'0', b'?') => Ordering::Less,
            | (b'?', b'0') => Ordering::Greater,
            | _ => unreachable!(),
        });
        let mut moves = 0;
        for (idx, _) in required {
            let Some(available_idx) = available.pop() else {
                break;
            };
            unsafe {
                let (available, required) =
                    (a.as_mut_ptr().add(available_idx), a.as_mut_ptr().add(idx));
                mem::swap(available.as_mut().unwrap(), required.as_mut().unwrap());
                moves += 1;
            }
        }
        for (input_by, target_by) in a.bytes().zip(b.bytes()) {
            if input_by == b'?' || input_by == b'0' && target_by == b'1' {
                moves += 1;
            } else if input_by != target_by {
                moves = -1;
                break;
            }
        }
        println!("Case {}: {}", i + 1, moves);
    }
}