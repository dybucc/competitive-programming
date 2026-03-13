use std::{
    cmp::Ordering,
    hint,
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
    // 10??01 -> 100001; ...
    //        -> 100101; ...
    //        -> 101001; ...
    //        -> 101101; ...
    // 100?01 -> 100001; ...
    //        -> 100101; ...
    //        -> 101001; ...
    //        -> 101101; ...
    // 11??01 -> 110001; ...
    //        -> 110101; ...
    //        -> 111001; ...
    //        -> 111101; ...
    // 11??11 -> 110001; ...
    //        -> 110101; ...
    //        -> 111001; ...
    //        -> 111101; ...
    // 10?1?1 -> 110001; ...
    //        -> 110101; ...
    //        -> 111001; ...
    //        -> 111101; ...
    // Test target 1: 100111
    // Test target 2: 000111
    // Test target 3: 110111
    for (i, [mut a, b]) in input.into_iter().enumerate() {
        let (mut available, mut required) = (Vec::new(), Vec::new());
        // Initial recon pass to gather values into the available and required
        // lists.
        a.bytes().zip(b.bytes()).enumerate().for_each(|(i, (input_by, target_by))| {
            if input_by == b'1' && target_by != b'1' {
                available.push(i);
            } else if matches!(input_by, b'0' | b'?') && target_by != b'0' {
                required.push((i, input_by));
            }
        });
        // Sorting of the required list in O(n log n).
        required.sort_unstable_by(|(_, by1), (_, by2)| match (by1, by2) {
            | (b'0', b'0') | (b'?', b'?') => Ordering::Equal,
            | (b'0', b'?') => Ordering::Less,
            | (b'?', b'0') => Ordering::Greater,
            | _ => unsafe { hint::unreachable_unchecked() },
        });
        let mut moves = 0;
        // Linear pass over the required list to swap/toggle/set bits to `1`.
        for (idx, _) in required {
            if let Some(available_idx) = available.pop() {
                // There's still elements in the available sequence.
                unsafe {
                    let (available, required) =
                        (a.as_mut_ptr().add(available_idx), a.as_mut_ptr().add(idx));
                    mem::swap(available.as_mut_unchecked(), required.as_mut_unchecked());
                    moves += 1;
                }
            } else {
                // There's no elements in the available sequence, so the only
                // thing that remains is resolution of `?` and `0` bytes.
                let required = unsafe { a.as_mut_ptr().add(idx) };
                unsafe { *required = b'1' };
                moves += 1;
            }
        }
        // Auxiliary linear pass if there were some `?` bytes in the input
        // sequence that didn't get resolved in the above linear pass.
        for (i, by) in unsafe { a.as_bytes_mut().iter_mut().enumerate() } {
            if *by == b'?' {
                *by = b.as_bytes()[i];
                moves += 1;
            }
        }
        // Final linear pass over the (now modified) input sequence, and the
        // target sequence looking for dissimilarities.
        for (input_by, target_by) in a.bytes().zip(b.bytes()) {
            if input_by != target_by {
                moves = -1;
                break;
            }
        }
        println!("Case {}: {}", i + 1, moves);
    }
}
