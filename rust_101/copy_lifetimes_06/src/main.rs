pub struct BigInt {
    pub data: Vec<u64>,
}

impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }
    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }
    pub fn from_vec(v: Vec<u64>) -> Self {
        BigInt { data: v }
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt {
            data: self.data.clone(),
        }
    }
}

impl BigInt {
    fn min_try1(self, other: Self) -> Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            unimplemented!()
        }
    }
}

fn vec_min(v: &Vec<BigInt>) -> Option<BigInt> {
    let mut min: Option<BigInt> = None;
    for e in v {
        let e = e.clone();
        min = Some(match min {
            None => e,
            Some(n) => e.min_try1(n),
        });
    }
    min
}

fn main() {
    println!("Hello, world!");
}
