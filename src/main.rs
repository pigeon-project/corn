#![feature(asm)]
#![feature(type_ascription)]
#![feature(const_fn)]
extern crate corn_cob;

use std::io;
use std::io::Write;
use corn_cob::parser::parse;
use corn_cob::context::CompileContext;
use corn_cob::preprocessor::preprocess;
use corn_cob::base_macro::macro_define_wrapper;
use corn_cob::utils::*;
use crate::corn_cob::base_macro::{load_prelude_macro};
use crate::corn_cob::preprocessor::{dyn_match, apply_macro};
use crate::corn_cob::context::{MacroDefine, PMNI};
use std::sync::Arc;
// use crate::corn_cob_o::context::{CompileContext, SExpr, CResult};


fn repl(compile_context: &CompileContext) -> ! {
    loop {
        io::stdout().write("λ ".as_ref()).unwrap();
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let r = parse(input.trim());
        println!("raw out: {:?}", r);
        if let Some(x) = r {
            let _: Vec<_> = x.iter()
                .map(|e| preprocess(compile_context, e))
                .map(|e| println!("macro-expand: {:?}", e)).collect();
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", &vec![1, 2, 3][3..3]);
    let compile_context= CompileContext::new();
    load_prelude_macro(&compile_context);
    println!("out: {:?}",
             preprocess(&compile_context,
                        &internal_parse_simple_expr("(macro t1 [(_ a) (*a)])")));
    // println!("out: {:?}",
    //          preprocess(&compile_context,
    //                     &internal_parse_simple_expr("(macro let [(_ ([var expr] ...) body ...) ((lambda (var ...) body ...) expr ...)])")));
    
    repl(&compile_context);
    // println!("out: {:?}",
    //          apply_macro(
    //              &compile_context,
    //              &Arc::new(MacroDefine::ProcessMacro(PMNI(String::from("macro"), nil(), macro_define))),
    //              &internal_parse_simple_expr(
    //                  "(t1 [1 2])")));
}

#[test]
fn macro_gulugulu() {
    preprocess(&mut Default::default(), &nil());
}