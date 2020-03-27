// use std::path::Display;
use std::sync::{Arc, RwLock};
use std::fmt::{Debug, Formatter};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
// use std::hash::{Hash, Hasher};


pub type Name = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Atom {
	Nil,
	Bool(bool),
	Int(i64),
	Uint(u64),
	Float(f32),
	Rational(i64, i64),
	Char(char),
	Str(String),
	Sym(Name),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SExpr {
	Atom(Atom),
	List(Vec<SExpr>),
	// Tuple(Vec<SExpr>),
	Pair(Box<(SExpr, SExpr)>)
}

impl SExpr {
	pub fn get_list(&self) -> &Vec<SExpr> {
		match self {
			SExpr::List(r) => r,
			_ => panic!("SExpr is not list")
		}
	}
}

/*#[derive(Debug, Serialize, Deserialize)]
pub enum Ast {
	Lit(Atom),
	Call(Vec<Ast>),
	Lambda(Vec<Name>, Vec<Ast>),
	Cond(Vec<(Ast, Ast)>, Option<Box<Ast>>)
}*/

// Process macro native interface
// #[derive(Clone)]

#[derive(Debug)]
pub struct CompileError ();
pub type CResult = Result<SExpr, CompileError>;
pub type MacroFun = fn(context: &CompileContext, sexprs: &SExpr) -> CResult;
pub struct PMNI (pub Name, pub SExpr, pub MacroFun);

impl Debug for PMNI {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "<process-macro '{}'>", self.0)
	}
}

#[derive(Debug)]
pub struct SyntaxRuleDefine (pub Vec<(SExpr, SExpr)>);

#[derive(Debug)]
pub enum MacroDefine {
	ProcessMacro(PMNI),
	SyntaxRule(SyntaxRuleDefine)
}

#[derive(Debug)]
pub struct FunctionDefine {

}

#[derive(Debug, Default)]
pub struct CompileContext {
	pub macro_defines   : RwLock<HashMap<String, Arc<MacroDefine>>>,
	pub function_defines: RwLock<HashMap<String, Arc<FunctionDefine>>>,
}
