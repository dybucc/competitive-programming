use std::{
    cmp::Ordering,
    hint,
    io::{self, BufRead, BufWriter, Write},
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
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct Item {
    name: String,
    class: [MaybeUninit<Class>; 10],
    init: u8,
}

impl Item {
    fn new(s: &str) -> Self {
        let mut comps = s.split_ascii_whitespace();
        let name = unsafe {
            comps
                .next()
                .map(|name| name.trim_end_matches(':'))
                .map(ToOwned::to_owned)
                .unwrap_unchecked()
        };
        let mut cont = const { [MaybeUninit::uninit(); 10] };
        Self {
            name,
            init: unsafe {
                comps
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
                    .unwrap_unchecked()
            },
            class: cont,
        }
    }
}

impl Ord for Item {
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

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Item {}

fn main() {
    let mut buf = String::with_capacity(const { 32 + 6 * 11 });
    let mut stdin = io::stdin().lock();
    let mut stdout = BufWriter::new(io::stdout().lock());
    let sep = const { [b'='; 30] };

    macro_rules! read {
        () => {
            unsafe { stdin.read_until(b'\n', buf.as_mut_vec()).unwrap_unchecked() }
        };
    }
    macro_rules! parse {
        () => {
            unsafe { buf.trim_ascii().parse().unwrap_unchecked() }
        };
    }

    read!();
    let mut cases: u16 = parse!();
    buf.clear();
    let mut itembuf = Vec::with_capacity(100);
    while cases != 0 {
        read!();
        let mut samples: u8 = parse!();
        buf.clear();
        while samples != 0 {
            read!();
            itembuf.push(Item::new(&buf));
            buf.clear();
            samples -= 1;
        }
        itembuf.sort_unstable();
        itembuf
            .iter()
            .rev()
            .for_each(|Item { name, .. }| unsafe { writeln!(stdout, "{name}").unwrap_unchecked() });
        unsafe {
            stdout.write_all(&sep).unwrap_unchecked();
            stdout.write_all(b"\n").unwrap_unchecked();
        };
        itembuf.clear();
        cases -= 1;
    }
    unsafe { stdout.flush().unwrap_unchecked() };
}
