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
        println!("{}", match WIN_POS.iter().try_fold(
          (0, 0),
          |(mut a_wins, mut b_wins), c| {
            ControlFlow::Continue(
              match c.iter().enumerate().fold(
                ([false; 3], [false; 3]),
                |(mut a_c, mut b_c), (i, c)| match i {
                  | 0..3 => (
                    (a_c[i] = a.contains(c), a_c).1,
                    (b_c[i] = b.contains(c), b_c).1,
                  ),
                  | _ => unreachable!(),
                },
              ) {
                | ([true, true, true], [false, false, false]) =>
                  (a_wins += 1, (a_wins, b_wins)),
                | ([false, false, false], [true, true, true]) =>
                  (b_wins += 1, (a_wins, b_wins)),
                | ([false, false, false], [false, false, false]) =>
                  ((), (a_wins, b_wins)),
                | _ => return ControlFlow::Break(()),
              }
              .1,
            )
          }
        ) {
          | ControlFlow::Continue((1, 0)) if a.len() == b.len() + 1 => "yes",
          | ControlFlow::Continue((0, 1)) if a.len() == b.len() => "yes",
          | ControlFlow::Continue((0, 0))
            if matches!(
              a.len(), n if n == b.len() || n == b.len() + 1
            ) =>
            "yes",
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
