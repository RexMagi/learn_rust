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

enum Variant {
    Number(i32),
    Text(String),
}

fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    *ptr = 1337
}

fn main() {
    println!("Hello, world!");
}
