use std::sync::Arc;
use super::preprocessor::dyn_match;
use crate::corn_cob::context::{Name, PMNI, MacroDefine, CompileContext, MacroFun, SExpr, CResult, CompileError};
use crate::corn_cob::parser::parse;
use crate::corn_cob::utils::nil;


fn register_macro(c: &CompileContext, k: Name, v: MacroDefine) {
	c.macro_defines
		.write()
		.unwrap()
		.insert(k.clone(), Arc::new(v));
}

fn register_native_macro(c: &CompileContext, k: Name, description: SExpr, v: MacroFun) {
	register_macro(c, k.clone(), MacroDefine::ProcessMacro( PMNI(k, description, v)));
}

pub fn internal_parse_simple_expr(input: &str) -> SExpr {
	parse(input).unwrap().get(0).unwrap().clone()
}

const MACRO_DEFINE_PATTERN: &'static str = "(&name ($+ [&pattern &template]))";

pub fn macro_define(context: &CompileContext, sexprs: &SExpr) -> CResult {
	let records = dyn_match(
		&internal_parse_simple_expr(MACRO_DEFINE_PATTERN),
		sexprs)?;
	println!("records: {:?}", records);
	Ok(nil())
}