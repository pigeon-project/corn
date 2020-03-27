use std::sync::Arc;
use std::collections::HashMap;
use std::cell::RefCell;
use super::context::{
	Name,
	SExpr,
	Atom::*,
	CResult,
	MacroDefine,
	CompileError,
	CompileContext };
use std::borrow::BorrowMut;
use std::iter::FromIterator;
// use super::utils::*;
// use crate::corn_cob::context::SExpr::Atom;

type MatchRecord = HashMap<Name, SExpr>;
type MatchResult = Result<MatchRecord, CompileError>;

fn merge_hash_table(r: Vec<MatchRecord>) -> MatchRecord {
	let mut record = HashMap::new();
	r.iter()
		.fold(&mut record,
		      |record, i| {
			      for (n, expr) in i { record.insert(n.clone(), expr.clone()); }
			      record
		      });
	return record;
}

pub fn dyn_match(pattern: &SExpr, target: &SExpr) -> MatchResult {
	println!("1: {:?}\n2: {:?}", pattern, target);
	match (pattern, target) {
		// 加值匹配
		(SExpr::List(s), SExpr::List(s1))
		if  s.len() >= 1 && s.len() <= s1.len() + 1 &&
			s.get(s.len()-1).map_or(false,|x| {
			if let SExpr::List(block) = x {
				if block.len() != 2 {
					return false;
				}
				if let SExpr::Atom(Sym(x)) = block.get(0).unwrap().clone() {
					x == "$*".to_string() || x == "$+".to_string()
				} else {
					false
				}
			} else {
				false
			}}) => {
			let r: Result<Vec<MatchRecord>, CompileError> = s[..s.len()-1].iter()
				.zip(s1[..s.len()-1].iter())
				.map(| (p, t) | dyn_match(p, t))
				.collect();
			let mut r = r?;
			if let SExpr::List(block) = s.get(s.len()-1).unwrap() {
				if let SExpr::Atom(Sym(x)) = block.get(0).unwrap().clone() {
					if x == "$+".to_string() && s.len()-1 == s1.len() {
						panic!("3");
						return Err(CompileError());
					}
					let pattern = s
						.get(s.len()-1)
						.expect("模式匹配里重复匹配少了最后一个").get_list()
						.clone();
					let pattern = pattern
						.get(1)
						.unwrap();
					println!("pattern: {:?}", pattern);
					let target: Result<Vec<MatchRecord>, CompileError> = s1[s.len()-1..]
						.iter()
						.map(|x| dyn_match(pattern, x))
						.collect();
					let target = target?;
					let mut r1: HashMap<Name, RefCell<Vec<SExpr>>> = HashMap::new();
					target
						.iter()
						.fold(&mut r1,  |records, x| {
							for (n, s) in x {
								match records.get(n) {
									Some(record) => {
										record.borrow_mut().push(s.clone());
									},
									None => {records.insert(n.clone(), RefCell::new(vec![s.clone()]));}
								}
							}
							records
						});
					let r1 = r1
						.into_iter()
						.map(|(n, y)| (n, SExpr::List(y.into_inner())));
					r.push(HashMap::from_iter(r1));
					Ok(merge_hash_table(r))
				} else {
					unreachable!()
				}
			} else {
				unreachable!()
			}
		},
		// 引用匹配任意量
		(SExpr::List(s), x)
		if s.len() == 2 && s.get(0).map_or(false,|x| {
			if let SExpr::Atom(Sym(s)) = x {
				s == "quote"
			} else { false }
		}) => {
			if s.len() != 2 {
				panic!("2");
				return Err(CompileError());
			}
			if let Some(SExpr::Atom(Sym(n))) = s.get(1) {
				let mut r = HashMap::new();
				r.insert(n.clone(), x.clone());
				Ok(r)
			} else {
				panic!("1");
				Err(CompileError())
			}
		}
		// 列表匹配
		(SExpr::List(x), SExpr::List(y)) => {
			let r: Result<Vec<MatchRecord>, CompileError> = x.iter()
				.zip(y.iter())
				.map(| (p, t) | dyn_match(p, t))
				.collect();
			Ok(merge_hash_table(r?))
		}
		// 字面量匹配
		(SExpr::Atom(x), SExpr::Atom(y)) =>
			if x == y { Ok(HashMap::new()) } else { Err(CompileError()) }
		_ => {panic!("4");Err(CompileError())}
	}
}

pub fn macro_expand(record: &HashMap<Name, SExpr>, template: &SExpr) -> CResult {
	match template {
		SExpr::List(l)
		if l.get(0).map_or(false,|x| {
			if let SExpr::Atom(Sym(s)) = x {
				s == "dequote"
			} else { false }
		})  => {
			if l.len() != 2 {
				panic!("5");
				return Err(CompileError());
			}
			if let Some(SExpr::Atom(Sym(n))) = l.get(1) {
				if let Some(r) = record.get(n) {
					Ok(r.clone())
				} else {
					panic!("6");
					Err(CompileError())
				}
			} else {
				panic!("7");
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

fn apply_macro(context: &CompileContext, macro_define: &Arc<MacroDefine>, sexprs: &SExpr) -> CResult {
	match &**macro_define {
		MacroDefine::ProcessMacro(fun) => (fun.2)(context, sexprs),
		MacroDefine::SyntaxRule(r) => {
			let r = r.0
				.iter()
				.map(|(pattern, temp)| (dyn_match(pattern, sexprs), temp));
			for (r, temp) in r {
				if let Ok(record) = r {
					return macro_expand(&record, temp);
				}
			}
			panic!("8");
			return Err(CompileError());
		}
	}
}

fn list_match(context: &CompileContext, list: &Vec<SExpr>) -> CResult {
	let r: Result<Vec<SExpr>, CompileError> = list
		.iter()
		.map(|l| preprocess(context, l))
		.collect();
	let r = r?;
	if let Some(SExpr::Atom(Sym(n))) = r.get(0) {
		if let Some(m) = context.macro_defines
			.read().unwrap()
			.get(n) {
			apply_macro(context, m, &SExpr::List(r[1..].to_vec()))
		} else {
			Ok(SExpr::List(r))
		}
	} else {
		Ok(SExpr::List(r))
	}
}

pub fn preprocess(context: &CompileContext, src: &SExpr) -> CResult {
	match src {
		SExpr::Atom(_) => Ok((*src).clone()),
		SExpr::Pair(_) => unimplemented!(),
		SExpr::List(r) => list_match(context, r)
	}
}