use std::sync::Arc;
use super::context::CompileContext;
use super::context::{ SExpr, Atom::* };
use super::utils::*;
use crate::corn_cob::context::MacroDefine;


fn dyn_match(c: &CompileContext, list: &Vec<SExpr>) -> SExpr {
	match list.as_slice() {
		_ => unimplemented!(),
	}
}

fn apply_macro(macro_define: &Arc<MacroDefine>, sexprs: &[SExpr]) -> SExpr {
	unimplemented!()
}

fn list_match(c: &CompileContext, list: &Vec<SExpr>) -> SExpr {
	let r: Vec<SExpr> = list
		.iter()
		.map(|l| macro_expand(c, l))
		.collect();
	if let Some(SExpr::Atom(Sym(n))) =  r.get(0) {
		if let Some(m) = c.macro_defines
			.read().unwrap()
			.get(n) {
			apply_macro(m, &r[1..])
		} else {
			SExpr::List(r)
		}
	} else {
		SExpr::List(r)
	}
}

pub fn macro_expand(c: &CompileContext, src: &SExpr) -> SExpr {
	match src {
		SExpr::Atom(_) => (*src).clone(),
		SExpr::Pair(_) => unimplemented!(),
		SExpr::List(r) => list_match(c, r)
	}
}