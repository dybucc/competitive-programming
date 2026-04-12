use std::{
    borrow::Cow,
    cmp::Ordering,
    convert::Infallible,
    io::{self, Read},
    iter,
    str::FromStr,
};

fn main() {
    #[derive(Debug)]
    enum Class {
        Lower(Option<Box<Self>>),
        Middle(Option<Box<Self>>),
        Upper(Option<Box<Self>>),
    }

    impl Class {
        fn new(input: impl AsRef<str>) -> Self {
            match input.as_ref() {
                "lower" => Self::Lower(None),
                "middle" => Self::Middle(None),
                "upper" => Self::Upper(None),
                _ => unreachable!(),
            }
        }

        fn set(&mut self, child: Self) {
            *self = match self {
                Self::Lower(_) => Self::Lower(Box::new(child).into()),
                Self::Middle(_) => Self::Middle(Box::new(child).into()),
                Self::Upper(_) => Self::Upper(Box::new(child).into()),
            };
        }
    }

    impl FromStr for Class {
        type Err = Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(if let Some((child, raw_class)) = s.rsplit_once('-') {
                let mut class = Self::new(raw_class);
                class.set(Self::from_str(child).unwrap());
                class
            } else {
                Self::new(s)
            })
        }
    }

    impl PartialEq for Class {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Lower(Some(class1)), Self::Lower(Some(class2)))
                | (Self::Middle(Some(class1)), Self::Middle(Some(class2)))
                | (Self::Upper(Some(class1)), Self::Upper(Some(class2))) => class1.eq(class2),
                (Self::Lower(_), Self::Lower(_))
                | (Self::Middle(_), Self::Middle(_))
                | (Self::Upper(_), Self::Upper(_)) => true,
                (Self::Lower(_) | Self::Upper(_), Self::Middle(_))
                | (Self::Lower(_) | Self::Middle(_), Self::Upper(_))
                | (Self::Middle(_) | Self::Upper(_), Self::Lower(_)) => false,
            }
        }
    }

    #[derive(Debug)]
    struct Item<'a> {
        name: Cow<'a, str>,
        class: Class,
    }

    impl Item<'_> {
        fn sort(
            Item {
                name: name1,
                class: class1,
            }: &Item,
            Item {
                name: name2,
                class: class2,
            }: &Item,
        ) -> Ordering {
            todo!()
        }
    }

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let cases: usize = lines.next().unwrap().parse().unwrap();
    let mut buf = Vec::new();
    let sep: String = iter::repeat_n('=', 30).collect();
    for _ in 0..cases {
        let len: usize = lines.next().unwrap().parse().unwrap();
        buf.reserve(len.saturating_sub(buf.capacity()));
        for _ in 0..len {
            let mut comps = lines.next().unwrap().split_ascii_whitespace();
            buf.push(Item {
                name: comps.next().unwrap().trim_end_matches(':').into(),
                class: Class::from_str(comps.next().unwrap()).unwrap(),
            });
        }
        buf.sort_unstable_by(Item::sort);
        for Item { name, .. } in &buf {
            println!("{name}");
        }
        println!("{sep}");
        buf.clear();
    }
}
