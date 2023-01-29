// from https://github.com/bign86/pest_latex

use std::fs::{self, File};
use std::io::{Write};
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};
use serde::Serialize;

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

#[derive(Debug, Serialize)]
//-- todo: probably add serde_derive
struct Node {
    children: Vec<Node>,
    _type: Token,
    value: String,
}

#[derive(Debug)]
enum Token {
    DocumentRoot,
    Section,
    Environment,
    Command,
    Literal,
}

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Token::DocumentRoot => serializer.serialize_str("document_root"),
            Token::Section => serializer.serialize_str("section"),
            Token::Environment => serializer.serialize_str("environment"),
            Token::Command => serializer.serialize_str("command"),
            Token::Literal => serializer.serialize_str("literal"),
        }
    }
}

const DEBUG: bool = false;
const SEPARATOR: &str = " | ";
const INPUT: &str = "latex_test.tex";
const OUTPUT: &str = "parsed.json";
fn main() {
    let src = fs::read_to_string(INPUT).expect("Cannot open file");

    let mut ast = Vec::<Node>::new();

    println!("parsing {}...", INPUT);
    let indent = 0;
    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            let mut n = Node {
                children: Vec::<Node>::new(),
                _type: Token::DocumentRoot,
                value: String::from(""),
            };

            for subpair in pair.into_inner() {
                if DEBUG {
                    println!("\n{:?}", subpair.as_rule());
                }

                match subpair.as_rule() {
                    Rule::section => {
                        let s = parse_section(subpair, indent);
                        if s.children.len() > 0 {
                            n.children.push(s);
                        }
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

    if DEBUG {
        pretty_print(&ast, 0);
    }

    let json_string = serde_json::to_string(&ast).unwrap();
    let mut output_file = File::create(OUTPUT).unwrap();
    match write!(output_file, "{}", json_string) {
        Ok(_) => println!("...wrote AST to {}", OUTPUT),
        Err(error) => println!("...failed to write to {}:{}", OUTPUT, error)
    }
}

fn pretty_print(_ast: &Vec<Node>, depth: usize) {
    for n in _ast.into_iter() {
        println!("{}type: {:?}", SEPARATOR.repeat(depth), n._type);
        println!("{}value: {}", SEPARATOR.repeat(depth), n.value);
        println!("{}children: {}", SEPARATOR.repeat(depth), n.children.len());
        pretty_print(&n.children, depth + 1);
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
            Rule::literal_group => {
                if DEBUG {
                    println!("{}literal: {}", SEPARATOR.repeat(indent), subpair.as_str());
                }

                section_node.children.push(Node {
                    _type: Token::Literal,
                    value: String::from(subpair.as_str()),
                    children: Vec::<Node>::new(),
                });
            }
            Rule::section => {
                let n = parse_section(subpair, indent);
                if n.children.len() > 0 {
                    section_node.children.push(n);
                }
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
                            if n.children.len() > 0 {
                                env_node.children.push(n);
                            }
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
            Rule::literal_group => {
                if DEBUG {
                    println!("{}literal: {}", SEPARATOR.repeat(indent), subpair.as_str());
                }

                cmd_node.children.push(Node {
                    _type: Token::Literal,
                    value: String::from(subpair.as_str()),
                    children: Vec::<Node>::new(),
                });
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
