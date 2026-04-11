use std::{
    borrow::Cow,
    convert::Infallible,
    io::{self, Read},
    str::FromStr,
};

fn main() {
    #[derive(Debug)]
    enum Class {
        Lower(Option<Box<Self>>),
        Middle(Option<Box<Self>>),
        Upper(Option<Box<Self>>),
    }

    impl FromStr for Class {
        type Err = Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut comps = s.split('-');
            let mut base_class = {
                let class = comps.next();
                match class.unwrap() {
                    "lower" => Class::Lower(None),
                    "middle" => Class::Middle(None),
                    "upper" => Class::Upper(None),
                    _ => unreachable!(),
                }
            };
            for comp in comps {
                let class = match comp {
                    "lower" => Class::Lower(None),
                    "middle" => Class::Middle(None),
                    "upper" => Class::Upper(None),
                    _ => unreachable!(),
                };
                todo!()
            }
            Ok(base_class)
        }
    }

    #[derive(Debug)]
    struct Item<'a> {
        name: Cow<'a, str>,
        class: Class,
    }

    let mut buf = String::new();
    let mut stdin = {
        let stdin = io::stdin();
        stdin.lock()
    };
    let res = stdin.read_to_string(&mut buf);
    res.unwrap();
    let mut lines = buf.lines();
    let cases = {
        let cases = lines.next();
        let cases = cases.unwrap();
        let res = cases.parse::<usize>();
        res.unwrap()
    };
    let mut buf = Vec::new();
    for _ in 0..cases {
        let len = {
            let len = lines.next();
            let len = len.unwrap();
            let res = len.parse::<usize>();
            res.unwrap()
        };
        buf.reserve(len);
        for _ in 0..len {
            let item = {
                let line = lines.next();
                line.unwrap()
            };
            let (name, class) = {
                let mut comps = item.split_ascii_whitespace();
                let name = {
                    let name = comps.next();
                    let name = name.unwrap();
                    name.trim_matches(':')
                };
                let class = {
                    let class = comps.next();
                    let class = class.unwrap();
                    let class = Class::from_str(class);
                    class.unwrap()
                };
                (name, class)
            };
            let name = name.into();
            let item = Item { name, class };
            buf.push(item);
        }
    }
}
