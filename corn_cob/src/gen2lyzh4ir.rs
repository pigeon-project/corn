use crate::context::*;
use crate::utils::{sym, ipse, UniqueID, concat_vec};
use crate::context::SExpr::List;
use crate::preprocessor::dyn_match;
use crate::context::TypeExpr::Built;

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

match_gen!(begin_match, "../meta_derive/begin.corn");
match_gen!(cond_match, "../meta_derive/cond.corn");
match_gen!(lambda_match, "../meta_derive/lambda.corn");
match_gen!(function_match, "../meta_derive/function.corn");


pub fn weak_type_inference(_expr: &SExpr) -> TypeExpr {
	//FIXME: implement
	TypeExpr::Built(BuiltinType::Top)
}

pub fn weak_type_check(expr: &SExpr, tp: &TypeExpr) -> Result<TypeExpr, CompileError> {
	weak_type_inference(expr).assert(tp)
}

trait Generable {
	type Output;
	fn gen2(self: Self, rc: &CompileContext) -> Self::Output;
}

type CodeGenResult = Result<(RuntimeContext, Vec<SExpr>), CompileError>;

lazy_static! {
	static ref COND_LABEL_NAME: UniqueID = Default::default();
}

fn begin_codegen(rc: &RuntimeContext, sexprs: &SExpr) -> CodeGenResult {
	let records = begin_match(sexprs)?;
	let exprs = records.1.get("expr").unwrap();
	let mut cutting_line = exprs.len()-1;
	for (sz, expr) in exprs.iter().enumerate() {
		if let TypeExpr::Built(BuiltinType::Top) = weak_type_inference(expr) {
			cutting_line = sz;
			break
		}
	}
	let exprs = &exprs[0..=cutting_line];
	let result = (exprs
		.iter()
		.fold(Ok((rc.clone(), Vec::new())),
		      |prev, expr| {
			      let (rc, prev_expr) = prev?;
			      let (rc, res) = base_codegen(&rc, expr)?;
			      Ok((rc, concat_vec(prev_expr, res)))
		      })
		.collect::<Result<Vec<_>, _>>()? as Vec<_>);
		// .reduce(|a, b| a + b) as Option<&Vec<SExpr>>
		// .map_or(vec![], |v| v.to_vec());
	Ok((rc.clone(), result))
}

fn cond_codegen(rc: &RuntimeContext, sexprs: &SExpr) -> CodeGenResult {
	let records = cond_match(sexprs)?;
	let boolexprs = records.1.get("boolexpr").unwrap();
	let exprs = records.1.get("expr").unwrap();
	let end_label: String = COND_LABEL_NAME.next();
	(boolexprs
		.iter()
		.map(|expr| weak_type_check(expr, &TypeExpr::Built(BuiltinType::Bool)))
		.collect::<Result<Vec<_>, _>>()? as Vec<_>)
		.iter()
		.zip(boolexprs.iter())
		.zip(exprs.iter())
		.fold(Ok((rc.clone(), Vec::new())),
			|result: CodeGenResult, ((tp, boolexpr), expr)| {
				let (rc, prev_expr) = result?;
				match tp {
					TypeExpr::Built(BuiltinType::Bot) =>
						base_codegen(&rc, boolexpr)
							.map(|(rc, res)| (rc, concat_vec(prev_expr, res))),
					TypeExpr::Built(tp) => {
						let (rc, mut r) = base_codegen(&rc, boolexpr)?;
						let (rc, code_block) = base_codegen(&rc, expr)?;
						let label: String = COND_LABEL_NAME.next();
						match tp {
							BuiltinType::Top => r.push(ipse("(dyn-cast builtin.bool)")),
							BuiltinType::Bool => (),
							_ => unreachable!()
						}
						r.push(ipse(&format!("(if-false-jump {})", label)));
						r.extend(code_block.into_iter());
						r.push(ipse(&format!("(jump {})", end_label)));
						r.push(ipse(&format!("(:label {})", label)));
						Ok((rc, r))
					},
					_ => unreachable!()
				}
			})
		.map(|(rt, mut r)| {
			r.push(ipse("(built-exception-throw \"Conditional branch is not captured\")"));
			r.push(ipse(&format!("(:label {})", end_label)));
			(rt, r)
		})
}

pub fn base_codegen(rc: &RuntimeContext, expr: &SExpr) -> CodeGenResult {
	match expr {
		SExpr::Atom(Atom::Sym(_name)) => Ok((rc.clone(), vec![List(vec![sym("load-sym"), expr.clone()])])),
		SExpr::Atom(_) => Ok((rc.clone(), vec![List(vec![sym("load-const"), expr.clone()])])),
		SExpr::List(_) =>
			if let Ok(r) = cond_codegen(rc, expr) {
				Ok(r)
			} else if let Ok(r) = begin_codegen(rc, expr) {
				Ok(r)
			} else {
				unreachable!()
			},
	}
}

/*
pub fn base_codegen_wrapper(_: &CompileContext, expr: &SExpr) -> CResult {
	base_codegen(expr)
}
*/


