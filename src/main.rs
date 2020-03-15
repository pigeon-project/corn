#![feature(asm)]
#![feature(type_ascription)]
#![feature(fixed_size_array)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod corn_kernals;

use corn_kernals::parser::parse;

fn main() {
    println!("Hello, world!");
    println!("out: {:?}", parse("(1 . 2)"))
}
