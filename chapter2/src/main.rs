use std::{
    cmp::Ordering,
    io::{self, Read, Write},
    mem::MaybeUninit,
    ptr,
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
                .map(|class| {
                    class
                        .split('-')
                        .rev()
                        .map(Class::new)
                        .zip(cont.iter_mut())
                        .fold(u8::default(), |init_elems, (class, cont)| {
                            cont.write(class);

                            init_elems + 1
                        })
                })
                .unwrap(),
            class: cont,
        }
    }
}

// TODO: keep looking into what's wrong with the total order defined here.
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

        macro_rules! init {
            (@sclass) => { *sinit };
            (@oclass) => { *oinit };
            ($cont:tt) => {{
                let bptr = $cont.as_ptr();
                unsafe {
                    ptr::slice_from_raw_parts(bptr, init!(@$cont) as usize)
                        .as_ref()
                        .unwrap()
                        .assume_init_ref()
                }
            }};
        }

        let mut sclass = init!(sclass).iter().peekable();
        let mut oclass = init!(oclass).iter().peekable();
        'a: loop {
            match sclass.next().cmp(&oclass.next()) {
                Ordering::Equal => {
                    if sclass.peek().is_some() && oclass.peek().is_some() {
                        continue;
                    }
                    'b: {
                        // TODO: tweak this to peek into the entirety of `sclass` as that seems to
                        // be the real order intended in the problem statement.
                        if let Some(s) = sclass.peek() {
                            match s {
                                Class::Middle => break 'b,
                                Class::Lower => break 'a Ordering::Less,
                                Class::Upper => break 'a Ordering::Greater,
                            }
                        } else if let Some(o) = oclass.peek() {
                            match o {
                                Class::Middle => break 'b,
                                Class::Lower => break 'a Ordering::Greater,
                                Class::Upper => break 'a Ordering::Less,
                            }
                        }
                    }
                    break sname.cmp(oname).reverse();
                }
                other => break other,
            }
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
        buf.clear();
    }
}
