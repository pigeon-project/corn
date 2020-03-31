use std::sync::Arc;
use super::preprocessor::dyn_match;
use crate::corn_cob::context::{Name, PMNI, MacroDefine, CompileContext, MacroFun, SExpr, CResult, CompileError, SyntaxRuleDefine};
use crate::corn_cob::parser::parse;
use crate::corn_cob::utils::nil;
use crate::corn_cob::context::Atom::*;


pub fn internal_parse_simple_expr(input: &str) -> SExpr {
	println!("Mr.P: {:?}", input);
	parse(input).unwrap().get(0).unwrap().clone()
}

// const MACRO_DEFINE_PATTERN: &'static str = "(name [pattern template] ...)";
const MACRO_DEFINE_PATTERN: &'static str = include_str!("meta_derive/macro.corn");

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
	context.register_macro(
		name,
		MacroDefine::SyntaxRule(
			SyntaxRuleDefine(name.clone(), macro_body)));
	Ok(nil())
}

#[inline]
fn macro_define_register__(context: &CompileContext) {
	context.register_native_macro(
		&"macro".to_string(),
		internal_parse_simple_expr(MACRO_DEFINE_PATTERN),
		macro_define);
}

pub fn load_prelude_macro(context: &CompileContext) {
	macro_define_register__(context);
}