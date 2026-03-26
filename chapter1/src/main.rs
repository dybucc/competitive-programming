use std::{
  collections::HashSet,
  io::{self, Read},
};

fn main() {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let mut lines = buf.lines();
  let (mut a, mut b, mut buf) = (
    HashSet::with_capacity(9),
    HashSet::with_capacity(9),
    HashSet::with_capacity(9),
  );
  let games = lines.next().unwrap().parse::<usize>().unwrap();
  (0..games).for_each(|i| {
    lines.next().unwrap();
    if i != games - 1 {
      (lines.next().unwrap(), ()).1
    }
    // TODO: perform actual cheking logic.
    (a.clear(), b.clear(), buf.clear(), ()).2
  });
}
