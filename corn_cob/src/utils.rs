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