use std::{
    collections::VecDeque,
    io::{self, Error, ErrorKind, Read},
};

fn main() {
    fn deq_copier(n: &VecDeque<u8>) -> &u8 {
        let last = n.back();
        last.unwrap()
    }
    fn copier(buf: &[u8]) -> &u8 {
        let first = buf.first();
        first.unwrap()
    }
    let mut stdin = io::stdin().lock();
    let mut buf = Vec::with_capacity(1);
    let mut n = VecDeque::new();
    buf.resize(1, 0);
    loop {
        let res = stdin.read_exact(&mut buf);
        match res {
            Ok(()) => {
                let byte = copier(&buf);
                if *byte == b'\n' {
                    break;
                }
                n.push_back(*byte);
            }
            _ => panic!(),
        }
    }
    buf.fill(0);
    let mut disp = 0;
    let map_kind = |e: Error| e.kind();
    let init_len = n.len();
    loop {
        let res = {
            let res = stdin.read_exact(&mut buf);
            res.map_err(map_kind)
        };
        match res {
            Ok(()) => {
                let byte = copier(&buf);
                if *byte != b'0' {
                    continue;
                }
                disp += 1;
                let last = deq_copier(&n);
                if *last == b'0' {
                    n.pop_back();
                }
                if disp > n.len() {
                    n.push_front(b'0');
                }
            }
            Err(ErrorKind::UnexpectedEof) => break,
            _ => panic!(),
        }
    }
    let current_len = n.len();
    if disp >= init_len {
        n.push_front(b'.');
        n.push_front(b'0');
    } else if init_len - disp < current_len {
        n.insert(init_len - disp, b'.');
    }
    let mut out = {
        let cap = n.len();
        String::with_capacity(cap)
    };
    let res = n.read_to_string(&mut out);
    res.unwrap();
    println!("{out}");
}
