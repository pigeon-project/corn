use pest::Parser;
use super::ast::{ Atom, SExpr };


#[derive(Parser)]
#[grammar = "corn_kernals/corn.pest"]
struct CornParser;


fn parser(src: &str) {
	let r: Result<_, _> = CornParser::parse(Rule::corn, src);
}