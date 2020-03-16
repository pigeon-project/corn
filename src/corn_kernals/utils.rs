use crate::corn_kernals::context::SExpr;
use crate::corn_kernals::context::Atom;

pub const fn nil() -> SExpr {
	SExpr::Atom(Atom::Nil)
}

pub fn sym(i: &str) -> SExpr {
	SExpr::Atom(Atom::Sym(String::from(i)))
}