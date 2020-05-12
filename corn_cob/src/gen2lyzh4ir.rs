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
match_gen!(while_match, "../meta_derive/while.corn");
// match_gen!(loop_match, "../meta_derive/loop.corn");
match_gen!(lambda_match, "../meta_derive/lambda.corn");
match_gen!(function_match, "../meta_derive/function.corn");


pub fn weak_type_inference(_expr: &SExpr) -> TypeExpr {
	//FIXME: implement
	TypeExpr::Built(BuiltinType::Top)
}

pub fn weak_type_check(expr: &SExpr, tp: &TypeExpr) -> Result<TypeExpr, CompileError> {
	weak_type_inference(expr).assert(tp)
}

// trait Generable {
// 	type Output;
// 	fn gen2(self: Self, rc: &CompileContext) -> Self::Output;
// }

type CodeGenResult = Result<(RuntimeContext, Vec<SExpr>), CompileError>;

lazy_static! {
	static ref COND_LABEL_NAME: UniqueID = Default::default();
}

fn next_name() -> String {
	COND_LABEL_NAME.next()
}

fn vec_expr_codegen(rc: &RuntimeContext, input: &[SExpr]) -> CodeGenResult {
	input
		.iter()
		.fold(Ok((rc.clone(), Vec::new())),
		      |prev, expr| {
			      let (rc, prev_expr) = prev?;
			      let (rc, res) = base_codegen(&rc, expr)?;
			      Ok((rc, concat_vec(prev_expr, res)))
		      })
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
	vec_expr_codegen(rc,exprs)
}

fn cond_codegen(rc: &RuntimeContext, sexprs: &SExpr) -> CodeGenResult {
	let records = cond_match(sexprs)?;
	let boolexprs = records.1.get("boolexpr").unwrap();
	let exprs = records.1.get("expr").unwrap();
	let end_label: String = next_name();
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
						let label: String = next_name();
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

fn lambda_codegen(rc: &RuntimeContext, sexprs: &SExpr) -> CodeGenResult {
	let records = lambda_match(sexprs)?;
	let vars = records.1.get("var").unwrap();
	let bodys = records.1.get("body").unwrap();
	let vars: Vec<Name> = vars.iter()
		.map(|this|
			match this {
				SExpr::Atom(Atom::Sym(x)) => Ok(x.clone()),
				_ => Err(CompileError())
			}).collect::<Result<Vec<Name>, CompileError>>()?;
	let (rc, bodys) = vec_expr_codegen(rc, bodys)?;
	let name = next_name();
	let funbody =
		FunctionDefine {
			name: name.clone(),
			pure_flag: None,
			typeinfo: None,
			args: vars,
			code_block: bodys
		};
	rc.register_function(&name, funbody);
	Ok((rc.clone(), vec![]))
}

fn function_codegen(rc: &RuntimeContext, sexprs: &SExpr) -> CodeGenResult {
	let records = function_match(sexprs)?;
	let name = records.0.get("name").unwrap();
	let name = match name {
		SExpr::Atom(Atom::Sym(x)) => x,
		_ => return Err(CompileError())
	};
	let vars = records.1.get("var").unwrap();
	let bodys = records.1.get("body").unwrap();
	let vars: Vec<Name> = vars.iter()
		.map(|this|
			match this {
				SExpr::Atom(Atom::Sym(x)) => Ok(x.clone()),
				_ => Err(CompileError())
			}).collect::<Result<Vec<Name>, CompileError>>()?;
	let (rc, bodys) = vec_expr_codegen(rc, bodys)?;
	let fun_body =
		FunctionDefine {
			name: name.clone(),
			pure_flag: None,
			typeinfo: None,
			args: vars,
			code_block: bodys
		};
	rc.register_function(name, fun_body);
	Ok((rc.clone(), vec![]))
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
			} else if let Ok(r) = lambda_codegen(rc, expr) {
				Ok(r)
			} else if let Ok(r) = function_codegen(rc, expr) {
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


