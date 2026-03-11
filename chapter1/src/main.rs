use std::io::{self, Read};

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
    // The approach conceived thus far attempts to compute the largest number
    // smaller than or equal to the target sequence, and _only then_ attempts to
    // perform bit swapping/toggling operations. An alternative would be to
    // reverse that order. Perform first a linear scan over both sequences,
    // keeping track of two lists. The first list should denote the byte indices
    // of `1` bytes "available," while the second list should denote the byte
    // indices of "required" `1` bytes. We define _available_ indices as those
    // where the input sequence is deemed to have a `1` byte where a `0` byte is
    // expected in the target sequence; Symmetrically, we define _required_
    // indices as those where the input sequence is outfit with a `0` byte where
    // the target sequence has a `1` byte. After the initial `O(n)` scan, the
    // latter list should be traversed, and should query the former on the
    // availability of some index, such that the byte at both positions may be
    // swapped. TODO: discuss the two possible outcomes of this and the final
    // `?`-byte resolution step.
    //
    // 10??01 -> 100001; ...
    //        -> 100101; ...
    //        -> 101001; ...
    //        -> 101101; ...
    // 11??01 -> 110001; ...
    //        -> 110101; ...
    //        -> 111001; ...
    //        -> 111101; ...
    // Test target 1: 100111
    // Test target 2: 000111
    // Test target 3: 110111
    for (i, [a, b]) in input.into_iter().enumerate() {
        for (i, b) in a.bytes().enumerate() {}
        println!("Case {}: {}", i + 1, -1);
    }
}
