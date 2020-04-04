use crate::context::*;
use crate::utils::{sym, internal_parse_simple_expr, ipse};
use crate::context::SExpr::List;
use crate::preprocessor::dyn_match;
use std::hint::unreachable_unchecked;

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

pub fn weak_type_check(expr: &SExpr, tp: &TypeExpr) -> Result<TypeExpr, CompileError> {
	weak_type_inference(expr).assert(tp)
}

type CodeGenResult = Result<(RuntimeContext, Vec<SExpr>), CompileError>;

fn cond_codegen(rc: &RuntimeContext, sexprs: &SExpr) -> CodeGenResult {
	let records = cond_match(sexprs)?;
	let boolexprs = records.1.get("boolexprs").unwrap();
	let exprs = records.1.get("exprs").unwrap();
	let code_list: Vec<Vec<SExpr>> = (boolexprs
		.iter()
		.map(|expr|
			weak_type_check(expr, &TypeExpr::Built(BuiltinType::Bool)))
		.collect::<Result<Vec<_>, _>>()? as Vec<_>)
		.iter()
		.zip(boolexprs.iter())
		.zip(exprs.iter())
		.map(|((tp, boolexpr), expr): ((&TypeExpr, &SExpr), &SExpr)|
			if let TypeExpr::Built(BuiltinType::Bot) = tp {
				base_codegen(rc, boolexpr)
			} else if let TypeExpr::Built(BuiltinType::Top) = tp {
				let mut r = base_codegen(rc, boolexpr)?.1;
				r.push(SExpr::List(vec![sym("dyn-cast"), sym("corn.builtin.bool")]));
				//FIXME: 未完成，半成品，map需要更新为fold来处理RuntimeContext
				// r.append();
				// (Default::default(), unimplemented!())
				unimplemented!()
			} else if let TypeExpr::Built(BuiltinType::Bool) = tp {
				unimplemented!()
			} else {
				unreachable!()
			})
		.collect::<Result<_, CompileError>>()?;
	unimplemented!()
}

pub fn base_codegen(rc: &RuntimeContext, expr: &SExpr) -> CodeGenResult {
	Ok((rc.clone(),
	    match expr {
		    SExpr::Atom(Atom::Sym(name)) => unimplemented!(),
		    SExpr::Atom(s) => vec![List(vec![sym("load-const"), expr.clone()])],
		    SExpr::List(_) =>
			    if let Ok(r) = cond_codegen(rc, expr) {
				    return Ok(r);
			    } else {
				    unreachable!()
			    },
		    _ => unreachable!()
	}))
}

/*
pub fn base_codegen_wrapper(_: &CompileContext, expr: &SExpr) -> CResult {
	base_codegen(expr)
}
*/
