use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    ops::Range,
};

// 1 3 2 4 -> 4 3 2 1
// 1a 3b 2c 4d -> 4d 3b 2c 1a: impossible
// 1 4 2 3 -> 4 3 2 1
// 1a 4b 2c 3d -> 4b 3d 2c 1a: possible
// 4 1 2 3 -> 4 3 2 1
// 4a 1b 2c 3d -> 4a 3d 2c 1b: impossible
// 4 2 3 1 -> 4 3 2 1
// 4a 2b 3c 1d -> 4a 3c 2b 1d: impossible
// 4 1 3 2 -> 4 3 2 1
// 4a 1b 3c 2d -> 4a 3c 2d 1b: impossible
// 4 3 1 2 -> 4 3 2 1
// 4a 3b 1c 2d -> 4a 3b 2d 1c: impossible
// 1 2 4 3 -> 4 3 2 1
// 1a 2b 4c 3d -> 4c 3d 2b 1a: impossible
fn dp(mut refer: [u32; 3], i: &[u32], f: &[u32], mut r: Range<usize>) -> bool {
    if i == f {
        return true;
    }
    let mut new_i = i.to_owned();
    for _ in 0..i.len() - 2 {
        let ss = &mut new_i[r.clone()];
        ss.rotate_right(1);
        if refer == ss {
            return false;
        }
        if dp(refer, &new_i, f, r.clone()) {
            return true;
        }
        if r.start < i.len() - 3 {
            r.start += 1;
            r.end += 1;
        } else {
            return false;
        }
        refer = *new_i[r.clone()].as_array().unwrap();
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
                .map(Result::unwrap)
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
    if dp(*i[..3].as_array().unwrap(), &i, &f, 0..3) {
        writeln!(stdout, "Possible").unwrap();
    } else {
        writeln!(stdout, "Impossible").unwrap();
    }
    stdout.flush().unwrap();
}
