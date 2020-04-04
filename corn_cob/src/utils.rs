use crate::context::SExpr;
use crate::context::Atom;
use crate::parser::parse;
use std::sync::{Mutex, RwLock};
use std::slice::SliceIndex;
use std::ops::AddAssign;

pub const fn nil() -> SExpr {
	SExpr::Atom(Atom::Nil)
}

pub fn sym(i: &str) -> SExpr {
	SExpr::Atom(Atom::Sym(String::from(i)))
}

#[inline]
pub fn internal_parse_simple_expr(input: &str) -> SExpr {
	println!("Mr.P: {:?}", input);
	parse(input).unwrap().get(0).unwrap().clone()
}

#[inline]
pub fn ipse(input: &str) -> SExpr {
	internal_parse_simple_expr(input)
}

pub fn get_next_id(record: &Mutex<usize>) -> usize {
	let result= *record.lock().unwrap();
	record.lock().unwrap().add_assign(1);
	return result;
}