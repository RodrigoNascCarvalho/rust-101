// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
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

    // We can convert any little-endian vector of digits (i.e., least-significant digit first) into a number,
    // by removing trailing zeros. The `mut` declaration for `v` here is just like the one in `let mut ...`:
    // We completely own `v`, but Rust still asks us to make our intention of modifying it explicit. This
    // `mut` is *not* part of the type of `from_vec` - the caller has to give up ownership of `v` anyway, so
    // they don't care anymore what you do to it.
    // 
    // **Exercise 05.1**: Implement this function.
    // 
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        v.reverse();
        let mut big = BigInt { data: v };

        while big.data.len() > 0 && big.data[big.data.len() - 1] == 0 {
            big.data.pop();
        }
        big
    }

    //EX 5.2
    pub fn print(&self) {
        let mut copy = self.data.clone();

        print!("BigInt: ");
        while let Some(value) = copy.pop() {
            print!("{}", value);
        }
        println!("");
    }

    pub fn digit_count(&self) -> usize {
        self.data.len()
    }

    pub fn min_digit(&self) -> Option<u64> {
        use std::cmp;

        let mut min = None;

        for digit in self.data.iter() {
            min = Some(match min {
                None => *digit,
                Some(value) => cmp::min(value, *digit)
            });
        }
        min
    }

    pub fn max_digit(&self) -> Option<u64> {
        use std::cmp;

        let mut max = None;

        for digit in self.data.iter() {
            max = Some(match max {
                None => *digit,
                Some(value) => cmp::max(value, *digit)
            });
        }
        max
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec(v.clone());
    let b2 = BigInt::from_vec(v);
}

/*impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone() }
    }
}*/

// We can also make the type `SomethingOrNothing<T>` implement `Clone`. 
use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref v) => Something(v.clone())
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the number of
// digits? The number of non-zero digits? The smallest/largest digit? Of course, these should all take `self` as a shared reference (i.e., in borrowed form).

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}

pub fn main() {
    //let v = vec![0,1 << 16];
    let v = vec![0,00,000000,1,2,112345678900,00033333,3,522222];

    let b1 = BigInt::from_vec(v.clone());

    b1.print();
    println!("Digit Count: {}", b1.digit_count());

    let min = match b1.min_digit() {
        None => 0,
        Some(value) => value
    };

    println!("Smallest Digit: {}", min);

    let max = match b1.max_digit() {
        None => 0,
        Some(value) => value
    };

    println!("Largest Digit: {}", max);
}
