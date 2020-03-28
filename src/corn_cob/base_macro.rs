use std::sync::Arc;
use super::preprocessor::dyn_match;
use crate::corn_cob::context::{Name, PMNI, MacroDefine, CompileContext, MacroFun, SExpr, CResult, CompileError, SyntaxRuleDefine};
use crate::corn_cob::parser::parse;
use crate::corn_cob::utils::nil;
use crate::corn_cob::context::Atom::*;


fn register_macro(c: &CompileContext, k: &Name, v: MacroDefine) {
	c.macro_defines
		.write()
		.unwrap()
		.insert(k.clone(), Arc::new(v));
}

fn register_native_macro(c: &CompileContext, k: &Name, description: SExpr, v: MacroFun) {
	register_macro(c, k, MacroDefine::ProcessMacro(PMNI(k.clone(), description, v)));
}

pub fn internal_parse_simple_expr(input: &str) -> SExpr {
	parse(input).unwrap().get(0).unwrap().clone()
}

const MACRO_DEFINE_PATTERN: &'static str = "(name ([pattern template] ...))";

pub fn macro_define(context: &CompileContext, sexprs: &SExpr) -> CResult {
	let records = dyn_match(
		&internal_parse_simple_expr(MACRO_DEFINE_PATTERN),
		sexprs)?;
	let name =
		if let SExpr::Atom(Sym(n)) = records.0.get("name").unwrap() { n }
		else { // type error
			return Err(CompileError());
		};
	let patterns = records.1.get("pattern").unwrap();
	let templates = records.1.get("template").unwrap();
	let macro_body: Vec<(SExpr, SExpr)> = patterns.into_iter()
		.zip(templates.iter())
		.map(|(pattern, temp)| (pattern.clone(), temp.clone()))
		.collect();
	register_macro(
		context,
		name,
		MacroDefine::SyntaxRule(
			SyntaxRuleDefine(name.clone(), macro_body)));
	Ok(nil())
}