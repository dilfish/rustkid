// sean at shanghai

use std::fmt::Display;

// generalizing functions
fn takes_anything<T>(x: T) {
    return;
}

fn takes_two<T>(x: T, y: T) {
    return;
}

fn takes_two_diff<T, U>(x: T, y: U) {
    return;
}

// generalizing struct
struct Point<T: Display> {
    x: T,
    y: T,
}

// gen_struct generate 2 types of point.int and point.float
fn gen_struct() {
    let point_a = Point { x: 0, y: 1 };
    let point_b = Point { x: 1.1, y: 2.2 };
    return;
}

// traits
trait Adder {
    fn add(&self);
}

// impl for traits
impl<T: Display> Adder for Point<T> {
    fn add(&self) {
        println!("{}", self.x);
    }
}

// functional programming

#[derive(Debug)]
// a and b live at least with States
struct States<'a> {
    a: &'a i32,
    b: &'a i32,
}

// return a function of i32->i32
trait Currying {
    type ReturnType: Fn(i32) -> i32;
    fn add(self) -> Self::ReturnType;
}

// put function in box
impl Currying for States<'static> {
    type ReturnType = Box<dyn Fn(i32) -> i32>;

    fn add(self) -> Self::ReturnType {
        Box::new(move |x| x * self.a)
    }
}

// combinators
fn combinator() {
    let vec = vec![1, 3, 4, 5, 6];
    // for very item in vector vec, plus 1 and construct a new vector
    let m = vec.iter().map(|&x| x + 1).collect::<Vec<i32>>();
    // zip, fold, max, all, flat_map
    println!("m is {:?}", m);
}

fn main() {
    let r_value: States = States { a: &100, b: &100 };

    let r1 = r_value.add();
    let r2 = r1(5);

    assert_eq!(500, r2);
    println!("Hello, world!");
}
