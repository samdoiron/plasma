extern crate pest;

use pest::Parser;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PlasmaParser;

#[derive(Debug, PartialEq, Eq)]
enum AstNode<'a> {
    Expressions(Vec<AstNode<'a>>),
    Assignment(&'a str, Box<AstNode<'a>>),
    Number(i64),
    Identifier(&'a str)
}

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

fn parse_number(pair: Pair) -> AstNode {
    AstNode::Number(pair.as_str().parse::<i64>().expect("parse number"))
}

fn parse_identifier<'a>(pair: Pair<'a>) -> AstNode<'a> {
    AstNode::Identifier(pair.as_str())
}

fn parse_assignment<'a>(pair: Pair<'a>) -> AstNode<'a> {
    let mut inner = pair.into_inner();
    let ident = inner.next().expect("identifier").as_str();
    let expression = parse_expression(inner.next().expect("expression"));
    AstNode::Assignment(ident, Box::new(expression))
}

fn parse_expression<'a>(expression: Pair<'a>) -> AstNode<'a> {
    match expression.as_rule() {
        Rule::number => parse_number(expression),
        Rule::identifier => parse_identifier(expression),
        Rule::assignment => parse_assignment(expression),
        other => panic!("Unexpected expression type: {:?}", other)
    }
}

fn parse<'a>(code: &'a str) -> AstNode<'a> {
    let mut pairs = PlasmaParser::parse(Rule::expressions, code)
        .expect("initial parse");

    let expressions = pairs.next()
        .expect("read expression");

    let expressions_vec = expressions
        .into_inner().into_iter()
        .filter_map(|expression| { 
            match expression.as_rule() {
                Rule::EOI => None,
                _ => Some(parse_expression(expression))
            }
        })
        .collect();

    AstNode::Expressions(expressions_vec)
}

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn it_parses_nothing() {
        assert_parses_to(
            "",
            AstNode::Expressions(vec![])
        );
    }

    #[test]
    fn it_parses_numbers() {
        assert_parses_to(
            "1337",
            AstNode::Expressions(vec![
                AstNode::Number(1337)
            ])
        );
    }

    #[test]
    fn it_parses_variables() {
        assert_parses_to(
            "foobar",
            AstNode::Expressions(vec![
                AstNode::Identifier("foobar")
            ])
        );
    }

    #[test]
    fn it_parses_assignments() {
        assert_parses_to(
            "foobar = 1337",
            AstNode::Expressions(vec![
                AstNode::Assignment("foobar", Box::new(AstNode::Number(1337)))
            ])
        );
    }

    fn assert_parses_to(input: &str, output: AstNode) {
        assert_eq!(parse(input), output);
    }
}
