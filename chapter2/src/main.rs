use std::{
  cmp::Ordering,
  io::{self, Read},
  iter,
};

fn main() {
  match iter::once(String::new())
    .map(|mut buf| {
      iter::once((io::stdin().read_to_string(&mut buf).unwrap(), buf).1)
        .map(|buf| {
          iter::once(buf.lines())
            .map(|mut lines| {
              (
                lines.next().unwrap().to_owned(),
                iter::once(lines.next().unwrap().to_owned())
                  .map(|mut buf| (buf.remove(0), buf).1)
                  .next()
                  .unwrap(),
              )
            })
            .next()
            .unwrap()
        })
        .next()
        .unwrap()
    })
    .next()
  {
    | Some((mut n, m)) => println!(
      "{}",
      match m.len().cmp(&n.len()) {
        // n: 924; 924; 9.24
        // m: 100;  00;   00
        | Ordering::Less => (n.insert(n.len() - m.len(), '.'), n).1,
        // n:  924; 924; 0.924
        // m: 1000; 000;   000
        | Ordering::Equal => (n.insert_str(0, "0."), n).1,
        // n:   924;  924; 0.0924
        // m: 10000; 0000;   0000
        | Ordering::Greater =>
          (
            n.insert_str(0, "0."),
            (0..m.len() - n.len()).fold(2, |s, i| (n.insert(s + i, '0'), s).1),
            n
          )
            .2,
      }
      .trim_end_matches('0')
      .trim_end_matches('.')
    ),
    | _ => unreachable!(),
  }
}
