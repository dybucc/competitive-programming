use std::{
  cmp,
  collections::HashSet,
  io::{self, Read},
};

fn main() {
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
            .for_each(|(j, _)| {
              $p.insert((j, i));
            })
        })
      }};
    }
    trav!(a);
    trav!(b);
    enum P {
      A,
      B,
    }
    use P::*;
    let (mut p, mut t, mut pt, mut fail) = (A, 1, 1, false);
    (0..queries).for_each(|_| {
      buf.clear();
      buf.extend(
        lines
          .next()
          .unwrap()
          .split_ascii_whitespace()
          .map(|n| n.parse::<usize>().unwrap()),
      );
      if matches!((a.len(), b.len()), (0, _) | (_, 0) if fail) {
        return;
      }
      let [x, y] = buf[..] else { unreachable!() };
      match p {
        | A if b.remove(&(x, y)) =>
          if b.is_empty() {
            p = B;
            t = pt;
            pt = 1;
          } else {
            pt += 1;
            t = cmp::max(t, pt);
          },
        | A => {
          p = B;
          t = pt;
          pt = 1;
          fail = true;
        },
        | B if a.remove(&(x, y)) =>
          if a.is_empty() {
            p = A;
            t = pt;
            pt = 1;
          } else {
            pt += 1;
            t = cmp::max(t, pt);
          },
        | B => {
          p = A;
          t = pt;
          pt = 1;
          fail = true;
        },
      }
    });
    println!("{}", match (a.len(), b.len()) {
      | (1.., 1..) | (0, 0) => "draw",
      | (1.., 0) => "player one wins",
      | (0, 1..) => "player two wins",
    });
    a.clear();
    b.clear();
    buf.clear();
  });
}
