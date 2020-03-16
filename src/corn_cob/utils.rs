use crate::corn_cob::context::SExpr;
use crate::corn_cob::context::Atom;

pub const fn nil() -> SExpr {
	SExpr::Atom(Atom::Nil)
}

pub fn sym(i: &str) -> SExpr {
	SExpr::Atom(Atom::Sym(String::from(i)))
}