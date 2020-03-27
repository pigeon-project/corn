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
// use super::utils::*;
// use crate::corn_cob::context::SExpr::Atom;

type MatchRecord = HashMap<Name, SExpr>;
type MatchResult = Result<MatchRecord, CompileError>;

fn dyn_match(pattern: &SExpr, target: &SExpr) -> MatchResult {
	match (pattern, target) {
		(SExpr::Atom(x), SExpr::Atom(y)) =>
			if x == y { Ok(HashMap::new()) } else { Err(CompileError()) }
		(SExpr::List(s), x)
		if s.get(0).map_or(false,|x| {
			if let SExpr::Atom(Sym(s)) = x {
				s == "quote"
			} else { false }
		}) => {
			if s.len() != 2 {
				return Err(CompileError());
			}
			if let Some(SExpr::Atom(Sym(n))) = s.get(1) {
				let mut r = HashMap::new();
				r.insert(n.clone(), x.clone());
				Ok(r)
			} else {
				Err(CompileError())
			}
		}
		(SExpr::List(x), SExpr::List(y)) => {
			let r: Result<Vec<MatchRecord>, CompileError> = x.iter()
				.zip(y.iter())
				.map(| (p, t) | dyn_match(p, t))
				.collect();
			let mut record = HashMap::new();
			r?.iter()
				.fold(&mut record,
				      |record, i| {
					      for (n, expr) in i { record.insert(n.clone(), expr.clone()); }
					      record
				      });
			Ok(record)
		}
		_ => unreachable!()
	}
}

fn macro_expand(record: &HashMap<Name, SExpr>, template: &SExpr) -> CResult {
	match template {
		SExpr::List(l)
		if l.get(0).map_or(false,|x| {
			if let SExpr::Atom(Sym(s)) = x {
				s == "dequote"
			} else { false }
		})  => {
			if l.len() != 2 {
				return Err(CompileError());
			}
			if let Some(SExpr::Atom(Sym(n))) = l.get(1) {
				if let Some(r) = record.get(n) {
					Ok(r.clone())
				} else {
					Err(CompileError())
				}
			} else {
				Err(CompileError())
			}
		}
		SExpr::List(l) => {
			let r: Result<Vec<SExpr>, CompileError> = l
				.iter()
				.map(|x| macro_expand(record, x))
				.collect();
			let r = r?;
			Ok(SExpr::List(r))
		}
		_ => Ok(template.clone())
	}
}

fn apply_macro(macro_define: &Arc<MacroDefine>, sexprs: &SExpr) -> CResult {
	match &**macro_define {
		MacroDefine::ProcessMacro(fun) => (fun.1)(sexprs),
		MacroDefine::SyntaxRule(r) => {
			let r = r.0
				.iter()
				.map(|(pattern, temp)| (dyn_match(pattern, sexprs), temp));
			for (r, temp) in r {
				if let Ok(record) = r {
					return macro_expand(&record, temp);
				}
			}
			return Err(CompileError());
		}
	}
}

fn list_match(c: &CompileContext, list: &Vec<SExpr>) -> CResult {
	let r: Result<Vec<SExpr>, CompileError> = list
		.iter()
		.map(|l| preprocess(c, l))
		.collect();
	let r = r?;
	if let Some(SExpr::Atom(Sym(n))) = r.get(0) {
		if let Some(m) = c.macro_defines
			.read().unwrap()
			.get(n) {
			apply_macro(m, &SExpr::List(r[1..].to_vec()))
		} else {
			Ok(SExpr::List(r))
		}
	} else {
		Ok(SExpr::List(r))
	}
}

pub fn preprocess(c: &CompileContext, src: &SExpr) -> CResult {
	match src {
		SExpr::Atom(_) => Ok((*src).clone()),
		SExpr::Pair(_) => unimplemented!(),
		SExpr::List(r) => list_match(c, r)
	}
}