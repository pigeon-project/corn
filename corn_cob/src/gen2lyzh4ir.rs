use crate::context::*;
use crate::utils::{sym, ipse, UniqueID, concat_vec};
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


pub fn weak_type_inference(_expr: &SExpr) -> TypeExpr {
	unimplemented!()
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

fn cond_codegen(rc: RuntimeContext, sexprs: &SExpr) -> CodeGenResult {
	let records = cond_match(sexprs)?;
	let boolexprs = records.1.get("boolexpr").unwrap();
	let exprs = records.1.get("expr").unwrap();
	(boolexprs
		.iter()
		.map(|expr|
			weak_type_check(expr, &TypeExpr::Built(BuiltinType::Bool)))
		.collect::<Result<Vec<_>, _>>()? as Vec<_>)
		.iter()
		.zip(boolexprs.iter())
		.zip(exprs.iter())
		.fold(Ok((rc, Vec::new())),
			|result: Result<(RuntimeContext, Vec<SExpr>), CompileError>, ((tp, boolexpr), expr)| {
				let (rc, prev_expr) = result?;
				match tp {
					TypeExpr::Built(BuiltinType::Bot) =>
						base_codegen(rc, boolexpr)
							.map(|(rc, res)| (rc, concat_vec(prev_expr, res))),
					TypeExpr::Built(tp) => {
						let (rc, mut r) = base_codegen(rc, boolexpr)?;
						let (rc, code_block) = base_codegen(rc, expr)?;
						let label: String = COND_LABEL_NAME.next();
						match tp {
							BuiltinType::Top => r.push(ipse("(dyn-cast builtin.bool)")),
							BuiltinType::Bool => (),
							_ => unreachable!()
						}
						r.push(ipse(&format!("(if-false-jump {})", label)));
						r.extend(code_block.into_iter());
						r.push(ipse(&format!("(:label {})", label)));
						Ok((rc, r))
					},
					_ => unreachable!()
				}
			}
		)
}

pub fn base_codegen(rc: RuntimeContext, expr: &SExpr) -> CodeGenResult {
	match expr {
		SExpr::Atom(Atom::Sym(_name)) => unimplemented!(),
		SExpr::Atom(_) => Ok((rc, vec![List(vec![sym("load-const"), expr.clone()])])),
		SExpr::List(_) =>
			if let Ok(r) = cond_codegen(rc, expr) {
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


