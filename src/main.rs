#![feature(asm)]
#![feature(type_ascription)]
#![feature(fixed_size_array)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod corn_kernals;

use std::io;
use corn_kernals::parser::parse;
use std::io::Write;

fn repl() -> ! {
    loop {
        io::stdout().write("λ ".as_ref()).unwrap();
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("out: {:?}", parse(input.trim()));
    }
}

fn main() {
    println!("Hello, world!");
    println!("out: {:?}", parse("&(+ \"str\\r\\n\" 's') 1 3.2 4/5 ([] . [])"));
    // repl()
}
