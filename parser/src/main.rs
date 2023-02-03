pub mod foliage;

use std::fs::{self, File};
use std::io::Write;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::{iterators::Pair, Parser};
use serde::Serialize;
use clap::Parser as ArgParser;
use crate::foliage::commands::parse_cmd_name;

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

#[derive(Debug, Serialize)]
struct Node {
    children: Vec<Node>,
    tag: Token,
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

const SEPARATOR: &str = " | ";
const DEFAULT_INPUT: &str = "latex_test.tex";
const DEFAULT_OUTPUT: &str = "parsed.json";
const DEFAULT_VERBOSE: usize = 0;
#[derive(ArgParser, Debug)]
struct Args {
    #[arg(short, default_value = DEFAULT_INPUT)]
    input: String,

    #[arg(short, default_value = DEFAULT_OUTPUT)]
    output: String,

    #[arg(short, default_value_t = DEFAULT_VERBOSE)]
    verbosity: usize,
}

fn main() {    
    let args = Args::parse();
    let src = fs::read_to_string(&args.input).expect("Cannot open file");
    let mut ast = Vec::<Node>::new();

    println!("reading: {}", args.input);
    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            let mut n = Node {
                children: Vec::<Node>::new(),
                tag: Token::DocumentRoot,
                value: String::from(""),
            };

            for subpair in pair.into_inner() {
                match subpair.as_rule() {
                    Rule::section => {
                        let s = parse_section(subpair);
                        if s.children.len() > 0 {
                            n.children.push(s);
                        }
                    }
                    Rule::env_stmt => {
                        let e = parse_environment(subpair);
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

    if args.verbosity == 1 {
        pretty_print(&ast, 0);
    }

    let json_string = serde_json::to_string(&ast).unwrap();
    match File::create(&args.output) {
        Ok(mut output_file) => {
            match write!(output_file, "{}", json_string) {
                Ok(_) => println!("writing: {}", args.output),
                Err(error) => println!("...failed to write {}:{}", args.output, error),
            }
        },
        Err(error) => println!("...failed to open {}:{}", args.output, error),
    }

}

fn pretty_print(_ast: &Vec<Node>, depth: usize) {
    for n in _ast.into_iter() {
        println!("{}tag: {:?}", SEPARATOR.repeat(depth), n.tag);
        println!("{}value: {}", SEPARATOR.repeat(depth), n.value);
        println!("{}children: {}", SEPARATOR.repeat(depth), n.children.len());
        pretty_print(&n.children, depth + 1);
    }
}

fn parse_section(_section: Pair<Rule>) -> Node {
    let mut section_node = Node {
        children: Vec::<Node>::new(),
        tag: Token::Section,
        value: String::from(""),
    };

    for subpair in _section.into_inner() {
        match subpair.as_rule() {
            Rule::env_stmt => {
                let e = parse_environment(subpair);
                section_node.children.push(e);
            }
            Rule::cmd_stmt => {
                if let Some(c) = parse_command(subpair) {
                    section_node.children.push(c)
                } else {
                    println!("skipping layout node")
                }
            }
            Rule::literal_group => {
                section_node.children.push(Node {
                    tag: Token::Literal,
                    value: String::from(subpair.as_str()),
                    children: Vec::<Node>::new(),
                });
            }
            Rule::section => {
                let n = parse_section(subpair);
                if n.children.len() > 0 {
                    section_node.children.push(n);
                }
            }
            Rule::COMMENT => println!("{}", subpair.as_str()),
            _ => unreachable!(),
        }
    }

    section_node
}

fn parse_environment(_env: Pair<Rule>) -> Node {
    let mut env_node = Node {
        children: Vec::<Node>::new(),
        tag: Token::Environment,
        value: String::from(""),
    };

    for subpair in _env.into_inner() {
        match subpair.as_rule() {
            Rule::name => env_node.value = String::from(subpair.as_str()),
            Rule::env_content => {
                for subsubpair in subpair.into_inner() {
                    match subsubpair.as_rule() {
                        Rule::section => {
                            let n = parse_section(subsubpair);
                            if n.children.len() > 0 {
                                env_node.children.push(n);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => println!("UNEXPECTED RULE {:?}", subpair.as_rule()),
        }
    }

    env_node
}

//-- parse_command can return None if the parsed Node is only related print layout
fn parse_command(_stmt: Pair<Rule>) -> Option<Node> {
    let mut cmd_node = Node {
        children: Vec::<Node>::new(),
        tag: Token::Command,
        value: String::from(""),
    };

    for subpair in _stmt.into_inner() {
        match subpair.as_rule() {
            Rule::ctrl_character => (),
            Rule::name => match parse_cmd_name(subpair.as_str()) {
                Some(cmd) => {
                    if cmd.is_print_layout() {
                        return None;
                    } else {
                        cmd_node.value = String::from(cmd.value());
                    }
                }
                None => panic!("Could not parse command: {}", subpair.as_str()),
            },
            Rule::cmd_stmt_opt => cmd_node.value = String::from(subpair.as_str()),
            Rule::literal_group => {
                cmd_node.children.push(Node {
                    tag: Token::Literal,
                    value: String::from(subpair.as_str()),
                    children: Vec::<Node>::new(),
                });
            }
            _ => println!("unexpected: {:?}", subpair.as_rule()),
        }
    }

    Some(cmd_node)
}
