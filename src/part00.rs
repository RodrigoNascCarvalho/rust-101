// Rust-101, Part 00: Algebraic datatypes
// ======================================

// As our first piece of Rust code, we want to write a function that computes the
// minimum of a list.

enum NumberOrNothing {
    Number(i32),
    Nothing
}

use self::NumberOrNothing::{Number,Nothing};

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing;

    for el in vec {
        match min {
            Nothing => {
                min = NumberOrNothing::Number(el);
            },
            Number(n) => {
                let new_min = min_i32(n, el);
                min = NumberOrNothing::Number(new_min);
            }
        }
    }
    return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn read_vec() -> Vec<i32> {
    vec![3, 4, 10, 1, 2, 5]
}

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("This array is empty"),
        Number(n) => println!("The minimum number value is {}", n)
    }
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}


