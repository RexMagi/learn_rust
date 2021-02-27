pub enum SomethingOrNothing<T: std::fmt::Display> {
    Something(T),
    Nothing,
}
pub trait Minimum: Copy {
    fn min(self, b: Self) -> Self;
}

impl<T: std::fmt::Display> SomethingOrNothing<T> {
    fn new(option: Option<T>) -> Self {
        match option {
            None => SomethingOrNothing::Nothing,
            Some(t) => SomethingOrNothing::Something(t),
        }
    }
    fn to_option(self) -> Option<T> {
        match self {
            Nothing => None,
            SomethingOrNothing::Something(t) => Some(t),
        }
    }

    fn print(&self) {
        match self {
            SomethingOrNothing::Nothing => println!("The number is: <nothing>"),
            SomethingOrNothing::Something(n) => println!("The number is: {}", n),
        }
    }
}

type NumberOrNothing = SomethingOrNothing<i32>;

impl NumberOrNothing {
    fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
        return SomethingOrNothing::new(Some(x));
    }
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b {
            self
        } else {
            b
        }
    }
}

pub fn vec_min<T: Minimum + std::fmt::Display>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = SomethingOrNothing::Nothing;
    for e in v {
        min = SomethingOrNothing::Something(match min {
            SomethingOrNothing::Nothing => e,
            SomethingOrNothing::Something(n) => e.min(n),
        });
    }
    min
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 3, 9, 27]
}

fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
