use std::{
  cmp::Ordering,
  io::{self, BufRead, ErrorKind, Read},
  iter,
};

fn main() {
  match iter::once((io::stdin().lock(), String::new()))
    .map(|(mut stdin, mut buf)| (stdin.read_line(&mut buf), (stdin, buf)).1)
    .next()
  {
    | Some((mut stdin, n)) => iter::once(
      iter::once(())
        .fold(Vec::with_capacity(1), |mut buf, _| (buf.resize(1, 0), buf).1),
    )
    .cycle()
    .try_for_each(|mut buf| {
      match match stdin.read_exact(&mut buf).map_err(|e| e.kind()) {
        | Err(ErrorKind::UnexpectedEof) => (_ = buf.pop(), None).1,
        | Err(_) => panic!(),
        | _ => buf.first(),
      } {
        | Some(b'0') => todo!(),
        | Some(_) => todo!(),
        | None => todo!(),
      }
    }),
    | _ => unreachable!(),
  };

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
        | Ordering::Greater => iter::once(
          (0..m.len() - n.len()).fold(
            iter::once(String::with_capacity(m.len() - n.len()))
              .map(|mut buf| (buf.push_str("0."), buf).1)
              .next()
              .unwrap(),
            |mut buf, _| (buf.push('0'), buf).1
          )
        )
        .map(|mut buf| (buf.push_str(&n), buf).1)
        .next()
        .unwrap(),
      }
      .trim_end_matches('0')
      .trim_end_matches('.')
    ),
    | _ => unreachable!(),
  }
}
