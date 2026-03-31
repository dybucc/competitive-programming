use std::{
  collections::HashSet,
  io::{self, Read},
  ops::ControlFlow,
};

fn main() {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut lines = buf.lines();
  let (mut a, mut b) = (HashSet::with_capacity(9), HashSet::with_capacity(9));
  (0..lines.next().unwrap().parse::<usize>().unwrap()).for_each(|_| {
    match (0..3).try_for_each(|i| {
      lines.next().unwrap().bytes().enumerate().try_for_each(
        |(j, byte)| match ((i, j), byte) {
          | ((3.., _) | (_, 3..), _) => ControlFlow::Break(()),
          | (_, b'X') => ControlFlow::Continue(_ = a.insert((i, j))),
          | (_, b'O') => ControlFlow::Continue(_ = b.insert((i, j))),
          | (_, b'.') => ControlFlow::Continue(()),
          | _ => ControlFlow::Break(()),
        },
      )
    }) {
      | ControlFlow::Break(()) =>
        (println!("no"), lines.next(), a.clear(), b.clear()),
      | _ => (
        println!("{}", match WIN_POS.iter().fold(
          (0, 0),
          |(mut a_wins, mut b_wins), [c0, c1, c2]| {
            (
              (a.contains(c0) && a.contains(c1) && a.contains(c2))
                .then(|| a_wins += 1),
              (b.contains(c0) && b.contains(c1) && b.contains(c2))
                .then(|| b_wins += 1),
              (a_wins, b_wins),
            )
              .2
          }
        ) {
          | (1, 0) if a.len() == b.len() + 1 => "yes",
          | (0, 1) if a.len() == b.len() => "yes",
          | (0, 0) if a.len() == b.len() || a.len() == b.len() + 1 => "yes",
          | _ => "no",
        }),
        lines.next(),
        a.clear(),
        b.clear(),
      ),
    };
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
