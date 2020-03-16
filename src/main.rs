#![feature(asm)]
#![feature(type_ascription)]
#![feature(const_fn)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod corn_cob;

use std::io;
use std::io::Write;
use corn_cob::parser::parse;
use corn_cob::preprocessor::macro_expand;
use crate::corn_cob::utils::nil;
use crate::corn_cob::context::CompileContext;

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
    // println!("out: {:?}", parse("&(+ \"str\\r\\n\" 's') 1 3.2 4/5 ([] . [])"));
    repl()
}

#[test]
fn macro_gulugulu() {
    use corn_cob::preprocessor;
    macro_expand(&Default::default(), &nil());
}