use std::io::{self, Read};

fn main() {
  let mut buf = String::new();
  io::stdin().read_to_string(&mut buf).unwrap();
  let (mut lines, mut buf, mut a, mut b) =
    (buf.lines(), Vec::with_capacity(3), Vec::new(), Vec::new());
  (0..lines.next().unwrap().parse::<usize>().unwrap()).for_each(|_| {
    buf.extend(
      lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap()),
    );
    let [xr, yr, queries] = buf[..] else {
      unreachable!();
    };
    // TODO: parsing and emulation logic
    (a.clear(), b.clear(), buf.clear());
  });
}
