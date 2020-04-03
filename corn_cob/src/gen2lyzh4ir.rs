use crate::context::*;
use crate::utils::{sym, internal_parse_simple_expr, ipse};
use crate::context::SExpr::List;
use crate::preprocessor::dyn_match;

struct IrSExpr (pub SExpr);

/*#[derive(Debug, Serialize, Deserialize)]
pub enum Ast {
	Lit(Atom),
	Call(Vec<Ast>),
	Lambda(Vec<Name>, Vec<Ast>),
	Cond(Vec<(Ast, Ast)>, Option<Box<Ast>>)
}*/

macro_rules! match_gen {
    ($name:ident, $src:expr) => {
        fn $name(sexprs: &SExpr) -> MatchResult {
			dyn_match(
				&ipse(include_str!($src)),
				sexprs)
		}
    };
}

match_gen!(cond_match, "../meta_derive/cond.corn");
match_gen!(lambda_match, "../meta_derive/lambda.corn");
match_gen!(function_match, "../meta_derive/function.corn");


pub fn weak_type_inference(expr: &SExpr) -> TypeExpr {
	unimplemented!()
}

type RuntimeContext = ();

#[inline]
pub fn base_codegen(expr: &SExpr) -> (RuntimeContext, CResult) {
	((), Ok(match expr {
		SExpr::Atom(s) => List(vec![sym("load-const"), expr.clone()]),
		_ => unimplemented!()
	}))
}

/*
pub fn base_codegen_wrapper(_: &CompileContext, expr: &SExpr) -> CResult {
	base_codegen(expr)
}
*/
