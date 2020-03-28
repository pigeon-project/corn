use std::sync::Arc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::borrow::{BorrowMut, Borrow};
use crate::corn_cob::context::{
	Name,
	SExpr,
	Atom::*,
	CResult,
	MacroDefine,
	CompileError,
	CompileContext,
	MatchRecord,
	MatchResult };
// use super::utils::*;
// use crate::corn_cob::context::SExpr::Atom;

fn merge_hash_table(r: &Vec<MatchRecord>) -> MatchRecord {
	let mut record = HashMap::new();
	r.iter()
		.fold(&mut record,
		      |record, MatchRecord(i, _)| {
			      for (n, expr) in i { record.insert(n.clone(), expr.clone()); }
			      record
		      });
	return MatchRecord(record, HashMap::new());
}

pub fn dyn_match(pattern: &SExpr, target: &SExpr) -> MatchResult {
	println!("1: {:?}\n2: {:?}", pattern, target);
	match (pattern, target) {
		// 匹配字面量
		(SExpr::List(s), x)
		if s.len() == 2 && s.get(0).map_or(false,|x| {
			if let SExpr::Atom(Sym(s)) = x {
				s == "quote"
			} else { false }
		}) => if x == s.get(1).unwrap() { Ok(MatchRecord(HashMap::new(), HashMap::new())) } else { Err(CompileError()) },
		// 列表匹配
		(SExpr::List(rx), SExpr::List(y)) if rx.len() -1 <= y.len() => {
			let x: &[SExpr];
			let mut endl: HashMap<Name, Vec<SExpr>>;
			match rx.get(rx.len()-1) {
				Some(SExpr::Atom(Sym(symbol))) if symbol == "..." => {
					// 获取拓展之前的表
					x = &rx[..rx.len() - 2];
					// 获取...之前的pattern
					let pattern = rx.get(rx.len()-2).unwrap();
					// 匹配模式对应的块们
					let r: Result<Vec<MatchRecord>, CompileError> = y[rx.len()-2..].iter()
						.map(|x| dyn_match(pattern, x))
						.collect();
					// 去皮
					let r = r?;
					// 获取变量匹配结果
					let mut records: HashMap<Name, RefCell<Vec<SExpr>>> = HashMap::new();
					r.iter().fold(&mut records, |records, i| {
						for (k, v) in i.0.iter() {
							if let Some(x) = records.get(k) {
								x.borrow_mut().push(v.clone());
							} else {
								records.insert(k.clone(), RefCell::new(vec![v.clone()]));
							}
						}
						records
					});
					// 削皮
					let mut records = records
						.into_iter()
						.map(|(k, v)| (k, v.into_inner()));
					// 包装end
					let mut records: HashMap<Name, Vec<SExpr>> = HashMap::from_iter(records);
					r.into_iter().fold((),|_, MatchRecord(_, end)| {
						for (k, v) in end {
							records.insert(k, v);
						}
					});
					endl = records;
				}
				_ => {x = &rx[..]; endl=HashMap::new()}
			}
			let r: Result<Vec<MatchRecord>, CompileError> = x.iter()
				.zip(y.iter())
				.map(| (p, t) | dyn_match(p, t))
				.collect();
			let r = r?;
			// let mut end_records: HashMap<Name, Vec<SExpr>> = HashMap::from_iter(records);
			r.iter().fold((),|_, MatchRecord(_, end)| {
				for (k, v) in end {
					endl.insert(k.clone(), v.clone());
				}
			});
			Ok(MatchRecord(merge_hash_table(&r).0, endl))
		}
		// 匹配词法变量
		(SExpr::Atom(Sym(n)), x) => {
			let mut r = HashMap::new();
			r.insert(n.clone(), x.clone());
			Ok(MatchRecord(r, HashMap::new()))
		}
		_ => {panic!("4");Err(CompileError())}
	}
}

pub fn macro_expand(record: &MatchRecord, template: &SExpr) -> CResult {
	match template {
		SExpr::List(l)
		if l.get(0).map_or(false,|x| {
			if let SExpr::Atom(Sym(s)) = x {
				s == "unquote"
			} else { false }
		})  => {
			if l.len() != 2 {
				panic!("5");
				return Err(CompileError());
			}
			if let Some(SExpr::Atom(Sym(n))) = l.get(1) {
				if let Some(r) = record.0.get(n) {
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
			let r = r.1
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