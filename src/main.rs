use chrono::Utc;
use std::time::Instant;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

pub fn main() {
    let result = fib(40);
    println!("{:?}", result);
}
