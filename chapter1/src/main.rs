use std::{
  collections::HashSet,
  io::{self, Read},
};

fn main() {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut lines = buf.lines();
  let (mut a, mut b) = (HashSet::with_capacity(9), HashSet::with_capacity(9));
  (0..lines.next().unwrap().parse::<usize>().unwrap()).for_each(|_| {
    (0..3).for_each(|i| {
      lines.next().unwrap().bytes().enumerate().for_each(
        |(j, byte)| match byte {
          | b'X' => _ = a.insert((i, j)),
          | b'O' => _ = b.insert((i, j)),
          | _ => (),
        },
      );
    });
    lines.next();
    println!("{}", match check(&a, &b) {
      | (true, Some(0)) if a.len() == b.len() + 1 => "yes",
      | (true, Some(1)) if a.len() == b.len() => "yes",
      | (true, None) if a.len() == b.len() || a.len() == b.len() + 1 => "yes",
      | _ => "no",
    });
    a.clear();
    b.clear();
  });
}

static WIN_POS: [[(usize, usize); 3]; 8] = [
  [(0, 0), (0, 1), (0, 2)],
  [(1, 0), (1, 1), (1, 2)],
  [(2, 0), (2, 1), (2, 2)],
  [(0, 0), (1, 0), (2, 0)],
  [(0, 1), (1, 1), (2, 1)],
  [(0, 2), (1, 2), (2, 2)],
  [(0, 0), (1, 1), (2, 2)],
  [(2, 0), (1, 1), (0, 2)],
];

fn check(
  a: &HashSet<(usize, usize)>,
  b: &HashSet<(usize, usize)>,
) -> (bool, Option<usize>) {
  let (a_wins, b_wins) =
    WIN_POS.iter().fold((0, 0), |(mut a_wins, mut b_wins), [c0, c1, c2]| {
      (
        (a.contains(c0) && a.contains(c1) && a.contains(c2))
          .then(|| a_wins += 1),
        (b.contains(c0) && b.contains(c1) && b.contains(c2))
          .then(|| b_wins += 1),
        (a_wins, b_wins),
      )
        .2
    });
  if matches!((a_wins, b_wins), (0, 0)) {
    return (true, None);
  }
  if (a_wins == 1) ^ (b_wins == 1) {
    return (true, Some(usize::from(a_wins != 1)));
  }
  (false, None)
}
