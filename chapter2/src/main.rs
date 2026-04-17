use std::{
    cmp::Ordering,
    convert::Infallible,
    fmt::{self, Display, Formatter},
    io::{self, Read, Write},
    str::FromStr,
};

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

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Lower(Some(class1)), Self::Lower(Some(class2)))
            | (Self::Middle(Some(class1)), Self::Middle(Some(class2)))
            | (Self::Upper(Some(class1)), Self::Upper(Some(class2))) => class1.cmp(class2),

            (Self::Lower(Some(class)), Self::Lower(None))
                if matches!(&**class, Class::Lower(_)) =>
            {
                Ordering::Less
            }
            (Self::Lower(Some(class)), Self::Lower(None))
                if matches!(&**class, Class::Upper(_)) =>
            {
                Ordering::Greater
            }

            (Self::Lower(None), Self::Lower(Some(class)))
                if matches!(&**class, Class::Lower(_)) =>
            {
                Ordering::Greater
            }
            (Self::Lower(None), Self::Lower(Some(class)))
                if matches!(&**class, Class::Upper(_)) =>
            {
                Ordering::Less
            }

            (Self::Middle(Some(class)), Self::Middle(None))
                if matches!(&**class, Class::Lower(_)) =>
            {
                Ordering::Less
            }
            (Self::Middle(Some(class)), Self::Middle(None))
                if matches!(&**class, Class::Upper(_)) =>
            {
                Ordering::Greater
            }

            (Self::Middle(None), Self::Middle(Some(class)))
                if matches!(&**class, Class::Lower(_)) =>
            {
                Ordering::Greater
            }
            (Self::Middle(None), Self::Middle(Some(class)))
                if matches!(&**class, Class::Upper(_)) =>
            {
                Ordering::Less
            }

            (Self::Upper(Some(class)), Self::Upper(None))
                if matches!(&**class, Class::Lower(_)) =>
            {
                Ordering::Less
            }
            (Self::Upper(Some(class)), Self::Upper(None))
                if matches!(&**class, Class::Upper(_)) =>
            {
                Ordering::Greater
            }

            (Self::Upper(None), Self::Upper(Some(class)))
                if matches!(&**class, Class::Lower(_)) =>
            {
                Ordering::Greater
            }
            (Self::Upper(None), Self::Upper(Some(class)))
                if matches!(&**class, Class::Upper(_)) =>
            {
                Ordering::Less
            }

            (Self::Lower(_), Self::Lower(_))
            | (Self::Middle(_), Self::Middle(_))
            | (Self::Upper(_), Self::Upper(_)) => Ordering::Equal,
            (Self::Lower(_), Self::Middle(_) | Self::Upper(_))
            | (Self::Middle(_), Self::Upper(_)) => Ordering::Less,
            (Self::Middle(_) | Self::Upper(_), Self::Lower(_))
            | (Self::Upper(_), Self::Middle(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Class {}

#[derive(Debug, PartialEq, Eq)]
struct ReverseShortlex<'a>(&'a str);

impl<'a> From<&'a str> for ReverseShortlex<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl Display for ReverseShortlex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Ord for ReverseShortlex<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(other.0).reverse()
    }
}

impl PartialOrd for ReverseShortlex<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Item<'a> {
    class: Class,
    name: ReverseShortlex<'a>,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let cases: usize = lines.next().map(str::parse).map(Result::unwrap).unwrap();
    let mut buf = Vec::new();
    let sep = [b'='; 30];
    let mut stdout = io::stdout().lock();
    for _ in 0..cases {
        let len: usize = lines.next().map(str::parse).map(Result::unwrap).unwrap();
        buf.reserve(len.saturating_sub(buf.capacity()));
        for _ in 0..len {
            let mut comps = lines.next().map(str::split_ascii_whitespace).unwrap();
            buf.push(Item {
                name: comps
                    .next()
                    .map(|comp| comp.trim_end_matches(':'))
                    .map(Into::into)
                    .unwrap(),
                class: comps
                    .next()
                    .map(Class::from_str)
                    .map(Result::unwrap)
                    .unwrap(),
            });
        }
        buf.sort_unstable();
        for Item { name, .. } in buf.iter().rev() {
            writeln!(stdout, "{name}").unwrap();
        }
        stdout.write_all(&sep).unwrap();
        stdout.write_all(b"\n").unwrap();
        buf.clear();
    }
}
