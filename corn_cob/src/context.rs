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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SExpr {
	Atom(Atom),
	List(Vec<SExpr>),
	// Tuple(Vec<SExpr>),
	Pair(Box<(SExpr, SExpr)>)
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

#[derive(Debug, Copy, Clone)]
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
	pub fn assert(&self, other: &Self) -> Result<Self, CompileError> {
		Ok(match (self, other) {
			(BuiltinType::Bot, _) |
			(_, BuiltinType::Top) => other.clone(),
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
	Apply(Vec<TypeExpr>),
}

impl TypeExpr {
	pub fn assert(&self, other: &Self) -> Result<Self, CompileError> {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct FunctionDefine {
	pub pure_flag: bool,
	// pub async_flag: bool,
	pub typeinfo: Vec<TypeExpr>,
	pub ast: Box<SExpr>,
}

#[derive(Debug)]
pub struct TypeDefine {
	pub record: HashMap<String, Arc<TypeExpr>>
}

#[derive(Debug, Default)]
pub struct CompileContext {
	pub macro_defines   : RwLock<HashMap<String, Arc<MacroDefine>>>,
	pub function_defines: RwLock<HashMap<String, Arc<FunctionDefine>>>,
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
	pub type_defines    : HashMap<String, Arc<TypeDefine>>,
	pub function_defines: HashMap<String, Arc<CodeBlock>>,
}

impl RuntimeContext {
	fn new() -> Self {
		Default::default()
	}
	
	fn find_function(&self, sym: &str) -> Option<&Arc<CodeBlock>> {
		self.function_defines.get(sym)
	}
}