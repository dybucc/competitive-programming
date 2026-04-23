use std::{
    cmp::Ordering,
    io::{self, Read, Write},
    mem::MaybeUninit,
    ops::ControlFlow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Class {
    Lower = 0,
    Middle = 1,
    Upper = 2,
}

impl Class {
    fn new(input: impl AsRef<str>) -> Self {
        match input.as_ref() {
            "lower" => Self::Lower,
            "middle" => Self::Middle,
            "upper" => Self::Upper,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct Item<'a> {
    name: &'a str,
    class: [MaybeUninit<Class>; 10],
    init: u8,
}

impl<'a> Item<'a> {
    fn new(s: &'a str) -> Self {
        let mut comps = s.split_ascii_whitespace();
        let name = comps.next().map(|name| name.trim_end_matches(':')).unwrap();
        let mut cont = const { [MaybeUninit::uninit(); 10] };
        Self {
            name,
            init: comps
                .next()
                .map(|chain| {
                    chain
                        .split('-')
                        .rev()
                        .map(Class::new)
                        .zip(cont.iter_mut())
                        .fold(u8::default(), |init, (class, cont)| {
                            cont.write(class);
                            init + 1
                        })
                })
                .unwrap(),
            class: cont,
        }
    }
}

impl Ord for Item<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let Self {
            name: sname,
            class: sclass,
            init: sinit,
        } = self;
        let Self {
            name: oname,
            class: oclass,
            init: oinit,
        } = other;

        macro_rules! iter {
            (@sclass) => { *sinit };
            (@oclass) => { *oinit };
            ($token:tt) => {
                unsafe { $token[..iter!(@$token) as usize].assume_init_ref().iter().peekable() }
            };
        }

        macro_rules! compare_last {
            (@sclass, $last:expr) => {
                break $last.cmp(&Class::Middle);
            };
            (@oclass, $last:expr) => {
                break Class::Middle.cmp($last);
            };
            ($iter:tt) => {{
                if let ControlFlow::Break(last) = $iter.try_for_each(|class| {
                    if matches!(class, Class::Middle) {
                        ControlFlow::Continue(())
                    } else {
                        ControlFlow::Break(class)
                    }
                }) {
                    compare_last!(@$iter, last);
                }
            }};
        }

        let mut sclass = iter!(sclass);
        let mut oclass = iter!(oclass);
        loop {
            match sclass.next().cmp(&oclass.next()) {
                Ordering::Equal if sclass.peek().is_some() && oclass.peek().is_some() => continue,
                Ordering::Equal if sclass.peek().is_some() => compare_last!(sclass),
                Ordering::Equal if oclass.peek().is_some() => compare_last!(oclass),
                Ordering::Equal => (),
                other => break other,
            }
            break sname.cmp(oname).reverse();
        }
    }
}

impl PartialOrd for Item<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl PartialEq for Item<'_> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Item<'_> {}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let cases: usize = lines.next().map(str::parse).map(Result::unwrap).unwrap();
    let mut buf = Vec::new();
    let sep = const { [b'='; 30] };
    let mut stdout = io::stdout().lock();
    for _ in 0..cases {
        let len: usize = lines.next().map(str::parse).map(Result::unwrap).unwrap();
        buf.reserve(len.saturating_sub(buf.capacity()));
        (0..len).for_each(|_| buf.push(Item::new(lines.next().unwrap())));
        buf.sort_unstable();
        buf.iter()
            .rev()
            .for_each(|Item { name, .. }| writeln!(stdout, "{name}").unwrap());
        stdout.write_all(&sep).unwrap();
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
        buf.clear();
    }
}
