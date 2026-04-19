use std::{
    cmp::Ordering,
    io::{self, Read, Write},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
struct Item<'a> {
    name: &'a str,
    class: Vec<Class>,
}

impl<'a> Item<'a> {
    fn new(s: &'a str) -> Self {
        let mut comps = s.split_ascii_whitespace();
        let name = comps.next().map(|name| name.trim_end_matches(':')).unwrap();
        Self {
            name,
            class: comps
                .next()
                .map(|class| class.split('-').map(Class::new).collect())
                .unwrap(),
        }
    }
}

// TODO: finish this and see if this `Ord` implementation is better than the
// recursive one we got before.
impl Ord for Item<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let Self {
            name: sname,
            class: sclass,
        } = self;
        let Self {
            name: oname,
            class: oclass,
        } = other;
        let mut sclass = sclass.iter().peekable();
        let mut oclass = oclass.iter().peekable();
        let class_cmp = loop {
            match sclass.next().cmp(&oclass.next()) {
                Ordering::Equal => {
                    if sclass.peek().is_some() && oclass.peek().is_some() {
                        continue;
                    }
                    if let Some(s) = sclass.peek()
                        && oclass.peek().is_none()
                    {
                        match s {
                            Class::Middle => break Ordering::Equal,
                            Class::Lower => break Ordering::Less,
                            Class::Upper => break Ordering::Greater,
                        }
                    }
                    if let Some(o) = oclass.peek()
                        && sclass.peek().is_none()
                    {
                        match o {
                            Class::Middle => break Ordering::Equal,
                            Class::Lower => break Ordering::Less,
                            Class::Upper => break Ordering::Greater,
                        }
                    }
                    break Ordering::Equal;
                }
                other => break other,
            }
        };
        if let Ordering::Equal = class_cmp {
            sname.cmp(oname).reverse()
        } else {
            class_cmp
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
