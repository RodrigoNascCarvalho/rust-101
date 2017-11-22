// Rust-101, Part 02: Generic types, Traits
// ========================================


// ## Generic datatypes

pub enum SomethingOrNothing<T>  {
    Something(T),
    Nothing,
}
// Instead of writing out all the variants, we can also just import them all at once.
pub use self::SomethingOrNothing::*;
type NumberOrNothing = SomethingOrNothing<i32>;
type FloatOrNothing = SomethingOrNothing<f32>;

// ## Generic `impl`, Static functions
// Inside an `impl`, `Self` refers to the type we are implementing things for. Here, it is
// an alias for `SomethingOrNothing<T>`.
impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o { None => Nothing, Some(t) => Something(t) }
    }

    fn to_option(self) -> Option<T> {
        match self { Nothing => None, Something(t) => Some(t) }
    }
}
// You can call static functions, and in particular constructors, as demonstrated in `call_constructor`.
fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

// ## Traits

pub trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            // Here, we can now call the `min` function of the trait.
            Something(n) => {
                e.min(n)
            }
        });
    }
    min
}

// ## Trait implementations
// To make `vec_min` usable with a `Vec<i32>`, we implement the `Minimum` trait for `i32`.
impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

// We again provide a `print` function.
impl NumberOrNothing {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}
// Now we are ready to run our new code. Remember to change `main.rs` appropriately.
fn read_vec() -> Vec<i32> {
    vec![18,5,7,3,9,27]
}

//EX 2.01 Minimum implementation for f32
fn read_vec_float() -> Vec<f32> {
    vec![18.5,5.1,7.2,3.32,9.321,27.11]
}

impl FloatOrNothing {
    pub fn print(self) {
        match self {
            Nothing => println!("The float number is: <nothing>"),
            Something(n) => println!("The float number is: {}", n)
        }
    }
}

impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();

    //f32 variant for EX 2.01
    let float_vec = read_vec_float();
    let min_float = vec_min(float_vec);
    min_float.print();
}


// **Exercise 02.1**: Change your program such that it computes the minimum of a `Vec<f32>` (where `f32` is the type
// of 32-bit floating-point numbers). You should not change `vec_min` in any way, obviously!

