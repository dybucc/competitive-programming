use std::io::{self,
              Read};
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
  // Permute however as many elements of the input string as there are `?`
  // bytes. This should yield `n` initial states, which already prune the search
  // space, as only 0s can be made into 1s, and from that point on, the search
  // for each individual state consists of all states derived from the initial
  // state where 0s are 1s, and all possible combinations (not permutations) of
  // the input sequence.
  //
  // Thus, for a `k`-long input sequence, where `1 <= k <= 100`, and there are
  // some `l` amount of `?` bytes, there should be `2^l` initial states, where
  // `0 <= l <= k`. Each of these ought bind its `move` counter to however as
  // many `?` bytes there are in the input sequence. Solving by actually
  // computing the permutation is bonkers, as the cost of this operation is
  // `Theta(2^l)`, where `0 <= l <= k, 1 <= k <= 100`, worst-case `2^100 =
  // 1.23e30`. Thus, computing through a complete search filter is infeasible,
  // and a recursive generator should be used instead.
  //
  // A recursive generator is only feasible if the computations can be made in
  // single-step fashion. The result of `2^l` ought be computed, but the
  // exhorbitant amount of bytes corresponding with all initial states need not
  // be stored prior to starting the complete search. Instead, the routine in
  // charge of recursing should behave in a fashion akin to that of iterator
  // adapters and short-circuiting iterator consumers; Namely, it should perform
  // only the computations regarding the "current" permutation result. This
  // would allow avoiding an immediate error regarding memory allocation limits,
  // as the worst case scenario would still only ever hold memory for a single
  // one of the `2^l` initial states, and its subsequent recursive stack frame
  // allocations.
  //
  // The above scheme would, in turn, require identifying the "current"
  // permutation. Because all initial states to which the input sequence
  // resolves to consist purely of `k`-element radix 2-encoded number sequences,
  // one can enumerate the permutations in terms of the decimal number to which
  // they map if the number were converted to base 10. For the bounds on `l`,
  // this would require a type capable of holding at least 100 bits. In Rust,
  // there exists only one such type, `{u,i}128`.
  //
  // A single instance of one such number would allow computing upwards of 100
  // elements, and thus would allow
  //
  // 10??01 -> 100001; ...
  //        -> 100101; ...
  //        -> 101001; ...
  //        -> 101101; ...
  // target: 100111
  for (i, [a, b]) in input.into_iter().enumerate() {
    for (i, b) in a.bytes().enumerate() {}
    println!("Case {}: {}", i + 1, -1);
  }
}
