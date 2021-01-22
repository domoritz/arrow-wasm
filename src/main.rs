extern crate arrow_wasm;
use std::fs;

fn main() {
    let data = fs::read("flights-10k.arrow").expect("Could not read file");

    println!("{}", arrow_wasm::parse(&data));
}
