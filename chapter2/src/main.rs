use std::{
    collections::VecDeque,
    io::{self, ErrorKind, Read},
};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = Vec::with_capacity(1);
    let mut n = VecDeque::new();
    buf.resize(1, 0);
    buf.clear();
    loop {
        let res = stdin.read_exact(&mut buf);
        match res.map_err(|e| e.kind()) {
            Err(ErrorKind::UnexpectedEof) => break,
            Ok(()) => {
                let byte = *buf.first().unwrap();
                n.push_back(byte);
                buf.clear();
            }
            _ => panic!(),
        }
    }
    buf.clear();
    let mut disp = 0;
    loop {
        let res = stdin.read_exact(&mut buf);
        match res.map_err(|e| e.kind()) {
            Err(ErrorKind::UnexpectedEof) => break,
            Ok(()) => {
                let byte = *buf.first().unwrap();
                if byte == b'0' {
                    disp += 1;
                }
                buf.clear();
            }
            _ => panic!(),
        }
    }
    if disp >= n.len() {
        let mut diff = disp - n.len();
        while diff > 0 {
            n.push_front(b'0');
            diff -= 1;
        }
        n.push_front(b'.');
        n.push_front(b'0');
    } else {
        n.insert(disp - 1, b'.');
    }
    let last = n.back();
    if let Some(last) = last
        && *last != b'0'
    {
        return;
    }
    let mut idx = None;
    for (i, byte) in n.iter().rev().copied().enumerate() {
        if byte != b'0' {
            idx = None;
            continue;
        }
        idx = i.into();
    }
    if let Some(idx) = idx {
        while idx != n.len() {
            n.swap_remove_back(idx);
        }
    }
}
