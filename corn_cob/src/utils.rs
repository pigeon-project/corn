use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use crate::context::SExpr;
use crate::context::Atom;
use crate::parser::parse;


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

pub fn concat_vec(mut a: Vec<SExpr>, b: Vec<SExpr>) -> Vec<SExpr> {
	a.extend(b.into_iter());
	a
}

#[derive(Debug, Default)]
pub struct UniqueID {
	current: AtomicU32,
}

impl UniqueID {
	pub fn new() -> UniqueID {
		Default::default()
	}

	pub fn next(&self) -> String {
		format!("unnamed-{}", self.current.fetch_add(1, Ordering::Relaxed))
	}
}

