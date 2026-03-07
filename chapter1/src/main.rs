use std::{
    array, cmp,
    io::{self, Read},
};

type Input = [(isize, isize, [isize; 100]); 100];

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut input = Vec::with_capacity(lines.next().unwrap().parse::<usize>().unwrap());
    let mut accum = ["nil"; 2];
    for (i, l) in lines.enumerate() {
        accum[i % 2] = l;
        if i != 0 && i % 2 != 0 {
            input.push(accum.map(ToString::to_string));
        }
    }
    let mut memo = [(isize::MAX, isize::MAX, [isize::MAX; 100]); 100];
    for (i, [a, b]) in input.into_iter().enumerate() {
        for (i, b) in a.bytes().enumerate() {
            let filler = |idx| if idx == i { -1 } else { isize::MAX };
            match b {
                b'0' => memo[i] = (-1, isize::MAX, array::from_fn(filler)),
                b'1' => memo[i] = (isize::MAX, isize::MAX, array::from_fn(filler)),
                b'?' => memo[i] = (isize::MAX, -1, array::from_fn(filler)),
                _ => unreachable!(),
            }
        }
        println!("Case {}: {}", i + 1, process(&a, memo));
    }
}

fn process(s: &str, mut memo: Input) -> isize {
    let mut min = isize::MAX;
    // ab? => baa; TODO: write down a proper backtrace of the recursion graph
    for i in 0..s.len() {
        for j in 0..memo[0].len() {
            if memo[i][j] != isize::MAX {
                memo[i][j] = cmp::min_by(
                    match j {
                        0 => process(
                            &[s.get(..i).unwrap(), "1", s.get(i..).unwrap()].concat(),
                            memo,
                        ),
                        1 => process(
                            &[s.get(..i).unwrap(), "0", s.get(i..).unwrap()].concat(),
                            memo,
                        ),
                        2 => process(
                            &[s.get(..i).unwrap(), "0", s.get(i..).unwrap()].concat(),
                            memo,
                        ),
                        _ => unreachable!(),
                    },
                    memo[i][j],
                    isize::cmp,
                );
            }
            min = cmp::min_by(memo[i][j], min, isize::cmp);
        }
    }

    min
}
