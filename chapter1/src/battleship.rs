use std::{
  collections::HashSet,
  io::{self, Read},
};

fn main() {
  enum P {
    A,
    B,
  }
  use P::{A, B};
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let (mut lines, mut buf, mut a, mut b) =
    (buf.lines(), Vec::with_capacity(3), HashSet::new(), HashSet::new());
  (0..lines.next().unwrap().parse::<usize>().unwrap()).for_each(|_| {
    buf.extend(
      lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap()),
    );
    let [_, yr, queries] = buf[..] else { unreachable!() };
    macro_rules! trav {
      ($p:expr) => {{
        (0..yr).rev().for_each(|i| {
          lines
            .next()
            .unwrap()
            .bytes()
            .enumerate()
            .filter(|(_, by)| *by == b'#')
            .for_each(|(j, _)| _ = $p.insert((j, i)))
        })
      }};
    }
    trav!(a);
    trav!(b);
    let (mut player, mut done) = (A, false);
    (0..queries).for_each(|_| {
      if done {
        return (_ = lines.next());
      }
      buf.clear();
      buf.extend(
        lines
          .next()
          .unwrap()
          .split_ascii_whitespace()
          .map(|n| n.parse::<usize>().unwrap()),
      );
      let [x, y] = buf[..] else { unreachable!() };
      match player {
        | A if b.remove(&(x, y)) => _ = b.is_empty().then(|| player = B),
        | A => player = B,
        | B if a.remove(&(x, y)) => _ = a.is_empty().then(|| done = true),
        | B => _ = (player = A, b.is_empty().then(|| done = true)),
      }
    });
    println!("{}", match (a.len(), b.len()) {
      | (0, 0) | (1.., 1..) => "draw",
      | (1.., 0) => "player one wins",
      | (0, 1..) => "player two wins",
    });
    a.clear();
    b.clear();
    buf.clear();
  });
}
