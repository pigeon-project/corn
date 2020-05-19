#![feature(asm)]
#![feature(type_ascription)]
#![feature(const_fn)]
extern crate corn_cob;

pub mod multitarget;

use std::io;
use std::io::Write;
use std::sync::Arc;

use corn_cob::parser::parse;
use corn_cob::context::{CompileContext, RuntimeContext, MacroDefine, PMNI, SExpr, CompileError};
use corn_cob::preprocessor::{dyn_match, apply_macro, preprocess};
use corn_cob::base_macro::{macro_define_wrapper, load_prelude_macro};
use multitarget::gen2lyzh4ir::base_codegen;
use corn_cob::utils::*;
