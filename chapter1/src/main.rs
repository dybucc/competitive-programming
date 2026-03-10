use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut input =
        Vec::with_capacity(lines.next().unwrap().parse::<usize>().unwrap());
    let mut accum = ["nil"; 2];
    for (i, l) in lines.enumerate() {
        accum[i % 2] = l;
        if i != 0 && i % 2 != 0 {
            input.push(accum.map(ToString::to_string));
        }
    }
    // For any input sequence `s`, for some `l` number of `?` bytes in `s`,
    // there exists `2^l` initial states from which to possibly derive some
    // output sequence `s'`, such that `s' = w`, for some sequence `w` where
    // `|w| = `s``.
    //
    // Furhter, the bounds on `|s|`, which from now on we shall refer to as `k`,
    // are such that for `1 <= k <= 100`, `0 <= l <= k`, which implies an upper
    // bound on the number of initial states of `2^l`, for `l = 100`, `2^(100) =
    // 1.23e30`.
    //
    // Thus, computing all initial states through a complete search filter is
    // infeasible, and one should instead attempt to solve the problem in terms
    // of a recursive complete search generator that pruned the search space
    // in-place.
    //
    // A possible alternative is to take care of the number that the binary
    // encoding provides, and prune the search space much earlier. This would
    // encompass taking into consideration the target sequence `w`, and
    // computing the integer value of the number denoted by it, `theta` from
    // here onwards. Then, one would have to compare `theta` with the input
    // sequence and resolve as the _single_ initial state, the largest number
    // smaller than or equal to `theta`. We will leave the details of that
    // procedure for later. Upon resolving an initial state, where no more moves
    // involving a replacement of `?` bytes is required, the algorithm should
    // enumerate all numbers between the one denoted by the initial state, from
    // here on referred to as `omega`, and the target sequence's numeric value,
    // namely `theta`. If any one of the resulting sequences, parsed from
    // computing any one of the above, matches a sequence of move operations
    // that forms a path between the initial state and the target sequence, then
    // a solution is feasible. Counting the number of steps in the path yields
    // the solution.
    //
    // The above formulation, even if unproven, does seem feasible, and
    // reduces the search space the moment it determines the largest number
    // smaller than or equal to the target sequence's representation.
    // Additionally, the final steps involving finding a path seem to fit nicely
    // with a graph problem, with as many vertices as elements in the initial
    // solution.
    //
    // 10??01 -> 100001; ...
    //        -> 100101; ...
    //        -> 101001; ...
    //        -> 101101; ...
    // Test target 1: 100111
    // Test target 2: 000111
    // Test target 3: 110111
    for (i, [a, b]) in input.into_iter().enumerate() {
        for (i, b) in a.bytes().enumerate() {}
        println!("Case {}: {}", i + 1, -1);
    }
}
