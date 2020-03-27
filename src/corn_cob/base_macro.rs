use std::sync::Arc;
use super::preprocessor;
use crate::corn_cob::context::{Name, PMNI, MacroDefine, CompileContext, MacroFun, SExpr, CResult, CompileError};


fn register_macro(c: &CompileContext, k: Name, v: MacroDefine) {
	c.macro_defines
		.write()
		.unwrap()
		.insert(k.clone(), Arc::new(v));
}

fn register_native_macro(c: &CompileContext, k: Name, description: SExpr, v: MacroFun) {
	register_macro(c, k.clone(), MacroDefine::ProcessMacro( PMNI(k, description, v)));
}

fn macro_define(context: &CompileContext, sexprs: &SExpr) -> CResult {
	match sexprs {
		SExpr::List(l) =>
			if let Some(SExpr::Atom(x)) = l.get(0) {
				unimplemented!()
			} else {
				Err(CompileError())
			},
		_ => Err(CompileError())
	}
}