// from https://github.com/bign86/pest_latex

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    _type: Token,
    value: String,
}

#[derive(Debug)]
enum Token {
    Document,
    Section,
    Environment,
    Command,
    Expression,
    Literal,
}

const DEBUG: bool = false;
const SEPARATOR: &str = " | ";
fn main() {
    let input = "latex_test.tex";
    let src = fs::read_to_string(input).expect("Cannot open file");

    let mut ast = Vec::<Node>::new();

    println!("parsing {}...", input);
    let indent = 0;
    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            let mut n = Node {
                children: Vec::<Node>::new(),
                _type: Token::Document,
                value: String::from(""),
            };

            for subpair in pair.into_inner() {
                println!("\n{:?}", subpair.as_rule());
                match subpair.as_rule() {
                    Rule::section => {
                        let s = parse_section(subpair, indent);
                        n.children.push(s);
                    }
                    Rule::env_stmt => {
                        let e = parse_environment(subpair, indent);
                        n.children.push(e);
                    }
                    // Rule::COMMENT => println!("{:?} -{}", subpair.as_rule(), subpair.as_str()),
                    Rule::EOI => (),
                    _ => println!("UNKNOWN {:?}", subpair.as_rule()),
                }
            }

            ast.push(n);
        }
        Err(error) => println!("error parsing: {}", error),
    }

    pretty_print(ast, 0);
}

fn pretty_print(_ast: Vec<Node>, depth: usize) {
    for n in _ast.into_iter() {
        println!("{}type: {:?}", SEPARATOR.repeat(depth), n._type);
        println!("{}value: {}", SEPARATOR.repeat(depth), n.value);
        pretty_print(n.children, depth + 1);
    }
}

fn parse_section(_section: Pair<Rule>, _indent: usize) -> Node {
    let mut section_node = Node {
        children: Vec::<Node>::new(),
        _type: Token::Section,
        value: String::from(""),
    };

    let indent = _indent + 1;
    for subpair in _section.into_inner() {
        if DEBUG {
            println!("{}{:?}", SEPARATOR.repeat(indent), subpair.as_rule());
        }

        match subpair.as_rule() {
            Rule::env_stmt => {
                let e = parse_environment(subpair, indent);
                section_node.children.push(e);
            }
            Rule::cmd_stmt => {
                let c = parse_cmd_stmt(subpair, indent);
                section_node.children.push(c);
            }
            Rule::expression => {
                let e = parse_expression(subpair, indent);
                section_node.children.push(e);
            }
            Rule::section => {
                let n = parse_section(subpair, indent);
                section_node.children.push(n);
            }
            Rule::COMMENT => println!("{}{}", SEPARATOR.repeat(indent), subpair.as_str()),
            _ => unreachable!(),
        }
    }

    section_node
}

fn parse_environment(_env: Pair<Rule>, _indent: usize) -> Node {
    let mut env_node = Node {
        children: Vec::<Node>::new(),
        _type: Token::Environment,
        value: String::from(""),
    };
    let indent = _indent + 1;
    for subpair in _env.into_inner() {
        match subpair.as_rule() {
            Rule::name => {
                if DEBUG {
                    println!("{}{}", SEPARATOR.repeat(indent), subpair.as_str());
                }
                env_node.value = String::from(subpair.as_str())
            }
            Rule::env_content => {
                if DEBUG {
                    println!("{}{:?}", SEPARATOR.repeat(indent), subpair.as_rule());
                }

                for subsubpair in subpair.into_inner() {
                    match subsubpair.as_rule() {
                        Rule::section => {
                            let n = parse_section(subsubpair, indent);
                            env_node.children.push(n);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => println!(
                "{}UNEXPECTED RULE {:?}",
                SEPARATOR.repeat(indent),
                subpair.as_rule()
            ),
        }
    }

    env_node
}

fn parse_cmd_stmt(_stmt: Pair<Rule>, _indent: usize) -> Node {
    let mut cmd_node = Node {
        children: Vec::<Node>::new(),
        _type: Token::Command,
        value: String::from(""),
    };

    let indent = _indent + 1;
    for subpair in _stmt.into_inner() {
        match subpair.as_rule() {
            Rule::ctrl_character => (),
            Rule::name => {
                if DEBUG {
                    println!("{}{}", SEPARATOR.repeat(indent), subpair.as_str());
                }

                cmd_node.value = String::from(subpair.as_str())
            }
            Rule::cmd_stmt_opt => {
                if DEBUG {
                    println!("{}{}", SEPARATOR.repeat(indent), subpair.as_str())
                }
                cmd_node.value = String::from(subpair.as_str())
            }
            Rule::expression => {
                let e = parse_expression(subpair, indent);
                cmd_node.children.push(e);
            }
            _ => println!(
                "{} unexpected: {:?}",
                SEPARATOR.repeat(indent),
                subpair.as_rule()
            ),
        }
    }

    cmd_node
}

fn parse_expression(_expr: Pair<Rule>, _indent: usize) -> Node {
    let mut expr_node = Node {
        children: Vec::<Node>::new(),
        _type: Token::Expression,
        value: String::from(""),
    };

    let indent = _indent + 1;
    for subpair in _expr.into_inner() {
        match subpair.as_rule() {
            Rule::cmd_stmt => {
                let c = parse_cmd_stmt(subpair, indent);
                expr_node.children.push(c);
            }
            Rule::literal => {
                if DEBUG {
                    println!("{}literal: {}", SEPARATOR.repeat(indent), subpair.as_str());
                }

                expr_node.children.push(Node {
                    _type: Token::Literal,
                    value: String::from(subpair.as_str()),
                    children: Vec::<Node>::new(),
                });
            }
            _ => unreachable!(),
        }
    }

    expr_node
}
