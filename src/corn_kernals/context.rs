use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type Name = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Ast {
	Lit(Atom),
	Call(Vec<Ast>),
	Lambda(Vec<Name>, Vec<Ast>),
	Cond(Vec<(Ast, Ast)>, Option<Box<Ast>>)
}


#[derive(Debug)]
pub enum MacroDefine {

}

#[derive(Debug)]
pub struct FunctionDefine {

}

#[derive(Debug, Default)]
pub struct CompileContext {
	pub macro_defines   : RwLock<HashMap<String, Arc<MacroDefine>>>,
	pub function_defines: RwLock<HashMap<String, Arc<FunctionDefine>>>,
}
