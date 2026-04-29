use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    ops::Range,
};

// 1 3 4 2
//   [1 3 4] 2
//     [4 1 3] 2
//       [3 4 1] 2
//     4 [1 3 2]
//       4 [2 1 3]
//         4 [3 2 1]
//   1 [3 4 2]
//     1 [2 3 4]
//       1 [4 2 3]
fn dp<'a>(mut refer: &'a [u32; 3], i: &'a [u32], f: &[u32], mut r: Range<usize>) -> bool {
    if i == f {
        return true;
    }
    for _ in 0..i.len() - 2 {
        let mut new = i.to_owned();
        let ss = &mut new[r.clone()];
        ss.rotate_right(1);
        if refer == ss {
            #[cfg(debug_assertions)]
            eprintln!("refer:\t{refer:?}, sequence repeated:\t{new:?}");
            return false;
        }
        #[cfg(debug_assertions)]
        eprintln!("refer:\t{refer:?}, sequence unrepeated:\t{new:?}");
        if dp(refer, &new, f, r.clone()) {
            return true;
        }
        if r.start < i.len() - 3 {
            r.start += 1;
        }
        if r.end < i.len() {
            r.end += 1;
        }
        refer = i[r.clone()].as_array().unwrap();
    }
    false
}

fn main() {
    let mut buf = String::new();
    let mut stdin = BufReader::new(io::stdin().lock());
    macro_rules! read {
        () => {
            unsafe { stdin.read_until(b'\n', buf.as_mut_vec()).unwrap() }
        };
    }
    macro_rules! c {
        ($b:expr) => {
            Vec::with_capacity($b)
        };
        () => {{
            let b = buf.trim_ascii().parse().unwrap();
            (c!(b), c!(b))
        }};
    }
    macro_rules! proc {
        ($c:expr) => {
            buf.split_ascii_whitespace()
                .map(str::parse::<u32>)
                .map(|res| res.unwrap())
                .for_each(|num| $c.push(num))
        };
    }
    read!();
    let (mut i, mut f) = c!();
    buf.clear();
    read!();
    proc!(i);
    buf.clear();
    read!();
    proc!(f);
    let mut stdout = BufWriter::new(io::stdout().lock());
    if dp(i[..3].as_array().unwrap(), &i, &f, 0..3) {
        stdout.write_all(b"Possible\n").unwrap();
    } else {
        stdout.write_all(b"Impossible\n").unwrap();
    }
    stdout.flush().unwrap();
}
