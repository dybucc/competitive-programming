use std::io::{self, BufRead};

fn dp(c: &[u32], i: &[u32], f: &[u32]) {}

fn main() {
    let mut buf = String::new();
    let mut stdin = io::stdin().lock();
    macro_rules! read {
        () => {
            unsafe { stdin.read_until(b'\n', buf.as_mut_vec()).unwrap_unchecked() }
        };
    }
    macro_rules! c {
        ($b:expr) => {
            Vec::with_capacity($b)
        };
        () => {{
            let b = unsafe { buf.trim_ascii().parse().unwrap_unchecked() };
            (c!(b), c!(b))
        }};
    }
    macro_rules! proc {
        ($c:expr) => {
            buf.split_ascii_whitespace()
                .map(str::parse::<u32>)
                .map(|res| unsafe { res.unwrap_unchecked() })
                .for_each(|num| $c.push(num))
        };
    }
    read!();
    let (mut i, mut f) = c!();
    read!();
    proc!(i);
    read!();
    proc!(f);
    dp(&i[..i.len() - 3], &i, &f);
}
