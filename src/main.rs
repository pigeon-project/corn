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
use corn_cob::context::CompileContext;
use corn_cob::preprocessor::preprocess;
use corn_cob::base_macro::macro_define;
use corn_cob::utils::nil;
use crate::corn_cob::base_macro::internal_parse_simple_expr;
use crate::corn_cob::preprocessor::dyn_match;
// use crate::corn_cob::context::{CompileContext, SExpr, CResult};


fn repl() -> ! {
    let mut compile_context: CompileContext = Default::default();
    loop {
        io::stdout().write("λ ".as_ref()).unwrap();
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let r = parse(input.trim());
        println!("raw out: {:?}", r);
        if let Some(x) = r {
            let _: Vec<_> = x.iter()
                .map(|e| preprocess(&mut compile_context, e))
                .map(|e| println!("macro-expand: {:?}", e)).collect();
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", &vec![1, 2, 3][3..3]);
    // println!("out: {:?}", parse("&(+ \"str\\r\\n\" 's') 1 3.2 4/5 ([] . [])"));
    // repl()
    // println!("out: {:?}",
    //          dyn_match(
    //              &internal_parse_simple_expr("(($+ &e))"),
    //              &internal_parse_simple_expr("(1 2)")));
    println!("out: {:?}",
             macro_define(
                 &Default::default(),
                 &internal_parse_simple_expr("(eval [true false] [false true])")));
}

#[test]
fn macro_gulugulu() {
    preprocess(&mut Default::default(), &nil());
}