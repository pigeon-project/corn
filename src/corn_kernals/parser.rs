use pest::Parser;
use pest::iterators::{Pair, Pairs};

use super::ast::{Atom, Atom::*, SExpr, SExpr::* };
// use super::ast::Ast;
use std::borrow::Borrow;


#[derive(Parser)]
#[grammar = "corn_kernals/corn.pest"]
struct CornParser;

fn escape_char_map(c: char) -> char {
	match c {
		'n' => '\n',
		'r' => '\r',
		't' => '\t',
		'\\' => '\\',
		'\'' => '\'',
		'\"' => '\"',
		_ => unreachable!()
	}
}

fn escape(s: &str) -> String {
	let mut ret_str = String::new();
	s.chars().fold(
		(&mut ret_str, false),
		|(ret_str, is_escape), i| {
			if is_escape {
				ret_str.push(escape_char_map(i));
				(ret_str, false)
			} else {
				if i == '\\' {
					(ret_str, true)
				} else {
					ret_str.push(i);
					(ret_str, false)
				}
			}
			
		}
	);
	ret_str
}

fn parse_atom(node: &Pair<Rule>) -> Atom {
	eprintln!("out: {}", node);
	eprintln!("span: {}", node.as_span().as_str());
	match node.as_rule() {
		Rule::nil   => Atom::Nil,
		Rule::bool  => Atom::Bool(
			if node.as_span().as_str() == "#f" { false } else { true }),
		Rule::int   => Atom::Int(node.as_span().as_str().parse().unwrap()),
		Rule::uint  => Atom::Uint(node.as_span().as_str().parse().unwrap()),
		Rule::float => Atom::Float(node.as_span().as_str().parse().unwrap()),
		Rule::char  => Atom::Char(*escape(
			&node
				.as_span()
				.as_str()
				.to_string()[1..node.as_span().as_str().len()-1])
			.chars()
			.collect::<Vec<char>>()
			.get(0)
			.unwrap()),
		Rule::rational => {
			let mut i = node.clone().into_inner();
			let l = i.next().unwrap();
			let r = i.next().unwrap();
			Atom::Rational(
				l.as_span().as_str().parse().unwrap(),
				r.as_span().as_str().parse().unwrap())
		}
		Rule::raw_str  => Atom::Str(node.as_span().as_str().parse().unwrap()),
		Rule::str   => Atom::Str(escape(node.as_span().as_str())),
		Rule::sym   => Atom::Sym(node.as_span().as_str().parse().unwrap()),
		_ => unreachable!()
	}
}

fn parse_sexpr(node: &Pair<Rule>) -> SExpr {
	match node.as_rule() {
		Rule::atom => SExpr::Atom(
			parse_atom(&node.clone()
				.into_inner()
				.next()
				.unwrap())),
		Rule::quote => List(vec![
			SExpr::Atom(Sym(String::from("quote"))),
			parse_sexpr(node.clone().into_inner().next().unwrap().borrow())
		]),
		Rule::pair => {
			let mut i = node.clone().into_inner();
			let l = i.next().unwrap();
			let r = i.next().unwrap();
			SExpr::Pair(Box::new((
				parse_sexpr(&l.into_inner().next().unwrap()),
				parse_sexpr(&r.into_inner().next().unwrap()))))
		}
		Rule::list =>
			List(
				node.clone()
					.into_inner()
					.map(|p| parse_sexpr(&p.into_inner().next().unwrap()))
					.collect()
			),
		_ => unreachable!()
	}
}

pub fn parse(src: &str) -> Option<Vec<SExpr>> {
	let r: Result<Pairs<Rule>, _> = CornParser::parse(Rule::corn, src);
	eprintln!("src result: {}", r.clone().ok()?);
	Some(r
		.ok()?
		.next().unwrap()
		.into_inner()
		.map(|node| parse_sexpr(
			&node
				.into_inner()
				.next().unwrap()))
		.collect())
}