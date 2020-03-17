use std::sync::Arc;
use std::collections::HashMap;
use super::context::{
	Name,
	SExpr,
	Atom::*,
	CResult,
	MacroDefine,
	CompileError,
	CompileContext };
use super::utils::*;


fn dyn_match(pattern: &[SExpr], target: &[SExpr]) -> HashMap<Name, SExpr> {
	unimplemented!()
}

fn apply_macro(macro_define: &Arc<MacroDefine>, sexprs: &[SExpr]) -> CResult {
	match &**macro_define {
		MacroDefine::ProcessMacro(fun) => (fun.1)(sexprs),
		MacroDefine::SyntaxRule(r) => {
			unimplemented!()
		}
	}
}

fn list_match(c: &CompileContext, list: &Vec<SExpr>) -> CResult {
	let r: Vec<CResult> = list
		.iter()
		.map(|l| macro_expand(c, l))
		.collect();
	let errlog = r
		.iter()
		.filter(|e| if let Err(_) = e { true } else { false }).collect::<Vec<_>>();
	if errlog.len() > 0 {
		//FIXME: 错误处理
		return Err(CompileError());
	}
	let r: Vec<SExpr> = r
		.into_iter()
		.map(|l| l.unwrap())
		.collect();
	if let Some(SExpr::Atom(Sym(n))) = r.get(0) {
		if let Some(m) = c.macro_defines
			.read().unwrap()
			.get(n) {
			apply_macro(m, &r[1..])
		} else {
			Ok(SExpr::List(r))
		}
	} else {
		Ok(SExpr::List(r))
	}
}

pub fn macro_expand(c: &CompileContext, src: &SExpr) -> CResult {
	match src {
		SExpr::Atom(_) => Ok((*src).clone()),
		SExpr::Pair(_) => unimplemented!(),
		SExpr::List(r) => list_match(c, r)
	}
}