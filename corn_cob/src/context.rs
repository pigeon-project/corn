// use std::path::Display;
use std::sync::{Arc, RwLock};
use std::fmt::{Debug, Formatter, Display};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use std::fmt;
use std::cell::RefCell;
// use std::hash::{Hash, Hasher};


pub type Name = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Atom {
	Nil,
	Bool(bool),
	Int(i64),
	Uint(u64),
	Float(f32),
	// Rational(i64, i64),
	Char(char),
	Str(String),
	Sym(Name),
}

impl ToString for Atom {
	fn to_string(&self) -> String {
		match self {
			Atom::Nil => "nil".to_string(),
			Atom::Bool(v) => v.to_string(),
			Atom::Int(v) => v.to_string(),
			Atom::Uint(v) => v.to_string(),
			Atom::Float(v) => v.to_string(),
			Atom::Char(v) => format!("'{}'", v),
			Atom::Str(v) => format!("\"{}\"", v),// FIXME: escape
			Atom::Sym(v) => v.to_string(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SExpr {
	Atom(Atom),
	List(Vec<SExpr>),
	// Tuple(Vec<SExpr>),
	// Pair(Box<(SExpr, SExpr)>)
}

impl ToString for SExpr {
	fn to_string(&self) -> String {
		match self {
			SExpr::Atom(a) => a.to_string(),
			SExpr::List(exprs) => format!("({})", exprs
				.iter()
				.fold(String::new(),
				      |mut res, e| {
					      if res.len() != 0 {
						      res.push(' ');
					      }
					      res.push_str(&e.to_string());
					      res
				      })),
		}
	}
}

impl SExpr {
	pub fn is_sym(&self) -> bool {
		if let SExpr::Atom(Atom::Sym(_)) = self {
			true
		} else {
			false
		}
	}
	
	pub fn get_list(&self) -> Vec<SExpr> {
		match self {
			SExpr::List(r) => r.clone(),
			_ => panic!("SExpr is not list")
		}
	}
	pub fn get_sym(&self) -> Name {
		match self {
			SExpr::Atom(Atom::Sym(r)) => r.clone(),
			_ => panic!("SExpr is not symbol")
		}
	}
}

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
pub struct MatchRecord (pub HashMap<Name, SExpr>, pub HashMap<Name, Vec<SExpr>>);
pub type MatchResult = Result<MatchRecord, CompileError>;

#[derive(Debug)]
pub struct SyntaxRuleDefine (pub Name, pub Vec<(SExpr, SExpr)>);

#[derive(Debug)]
pub enum MacroDefine {
	ProcessMacro(PMNI),
	SyntaxRule(SyntaxRuleDefine)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BuiltinType {
	Top,
	Bot,
	Unit,
	Bool,
	Char,
	U8,
	U32,
	U64,
	I32,
	I64,
	F32,
	F64,
	Str,
	Arr,
}

impl BuiltinType {
	/*
	pub fn assert(&self, other: &Self) -> Result<Self, CompileError> {
		if self == other {
			Ok(self.clone())
		} else {
			Err(CompileError())
		}
	}
	*/
	pub fn assert(&self, other: &Self) -> Result<Self, CompileError> {
		Ok(match (self, other) {
			// (BuiltinType::Top, _) => other.clone(),
			(BuiltinType::Top, _) |
			(BuiltinType::Bot, _) |
			(BuiltinType::Unit, BuiltinType::Unit) |
			(BuiltinType::Bool, BuiltinType::Bool) |
			(BuiltinType::Char, BuiltinType::Char) |
			(BuiltinType::U8, BuiltinType::U8) |
			(BuiltinType::U32, BuiltinType::U32) |
			(BuiltinType::U64, BuiltinType::U64) |
			(BuiltinType::I32, BuiltinType::I32) |
			(BuiltinType::I64, BuiltinType::I64) |
			(BuiltinType::F32, BuiltinType::F32) |
			(BuiltinType::F64, BuiltinType::F64)=> self.clone(),
			_ => return Err(CompileError())
		})
	}
}

#[derive(Debug, Clone)]
pub enum TypeExpr {
	Built(BuiltinType),
	Tuple(Vec<TypeExpr>),
	Union(Vec<TypeExpr>),
	Arrow(Vec<TypeExpr>),
	Aplay(Name, Vec<TypeExpr>),
}

impl TypeExpr {
	pub fn assert(&self, other: &Self) -> Result<Self, CompileError> {
		match (self, other) {
			(TypeExpr::Built(t1), TypeExpr::Built(t2)) =>
				t1.assert(t2)
					.map(|e| TypeExpr::Built(e)),
			_ => unimplemented!(),
		}
	}
}

#[derive(Debug)]
pub struct FunctionDefine {
	pub name: Name,
	pub pure_flag: Option<bool>,
	// pub async_flag: bool,
	pub typeinfo: Option<TypeExpr>,
	pub args: Vec<Name>,
	pub code_block: Vec<SExpr>,
}

#[derive(Debug)]
pub struct TypeDefine {
	pub record: HashMap<String, Arc<TypeExpr>>
}

#[derive(Debug, Default)]
pub struct CompileContext {
	pub macro_defines   : RwLock<HashMap<String, Arc<MacroDefine>>>,
	// pub function_defines: RwLock<HashMap<String, Arc<FunctionDefine>>>,
	pub type_defines    : RwLock<HashMap<String, Arc<TypeDefine>>>,
}

impl CompileContext {
	pub fn new() -> Self {
		Default::default()
	}
	
	pub fn register_macro(&self, k: &Name, v: MacroDefine) {
		self.macro_defines
			.write()
			.unwrap()
			.insert(k.clone(), Arc::new(v));
	}
	
	pub fn register_native_macro(&self, k: &Name, description: SExpr, v: MacroFun) {
		self.register_macro(k, MacroDefine::ProcessMacro(PMNI(k.clone(), description, v)));
	}
}

#[derive(Clone, Default)]
pub struct CodeBlock {
	pub pure_flag: bool,
	// pub async_flag: bool,
	pub typeinfo: Vec<TypeExpr>,
	pub code: Vec<SExpr>,
}

#[derive(Clone, Default)]
pub struct RuntimeContext {
	pub type_defines    : RefCell<HashMap<String, Arc<TypeDefine>>>,
	pub function_defines: RefCell<HashMap<String, Arc<FunctionDefine>>>,
}

impl RuntimeContext {
	fn new() -> Self {
		Default::default()
	}
	
	pub fn register_function(&self, k: &Name, v: FunctionDefine) {
		self.function_defines
			.borrow_mut()
			.insert(k.clone(), Arc::new(v));
	}
	
	fn find_function(&self, sym: &str) -> Option<Arc<FunctionDefine>> {
		self.function_defines
			.borrow()
			.get(sym)
			.map(|x|x.clone())
	}
}