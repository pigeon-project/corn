use std::sync::Arc;
use crate::parser::parse;
use crate::context::{
	Name,
	PMNI,
	MacroDefine,
	CompileContext,
	MacroFun,
	SExpr,
	CResult,
	CompileError,
	SyntaxRuleDefine,
	Atom::*
};
use super::preprocessor::dyn_match;
use crate::utils::{nil, internal_parse_simple_expr, ipse};

// const MACRO_DEFINE_PATTERN: &'static str = "(name [pattern template] ...)";
const MACRO_DEFINE_PATTERN: &'static str = include_str!("../meta_derive/macro.corn");

pub fn macro_define_wrapper(context: &CompileContext, sexprs: &SExpr) -> CResult {
	let records = dyn_match(
		&ipse(MACRO_DEFINE_PATTERN),
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
fn macro_define_wrapper_register__(context: &CompileContext) {
	context.register_native_macro(
		&"macro".to_string(),
		internal_parse_simple_expr(MACRO_DEFINE_PATTERN),
		macro_define_wrapper);
}

pub fn load_prelude_macro(context: &CompileContext) {
	macro_define_wrapper_register__(context);
}