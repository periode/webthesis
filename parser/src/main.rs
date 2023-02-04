pub mod foliage;

use std::fs::{self, File};
use std::io::Write;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use clap::Parser as ArgParser;
use foliage::environments::Environment;
use foliage::tokens::Token;
use foliage::{commands, environments, Tag};
use pest::{iterators::Pair, Parser};
use serde::Serialize;

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

#[derive(Serialize)]
struct Node {
    children: Vec<Node>,
    tag: Box<dyn Tag>,
    value: String,
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
    println!("reading: {}", args.input);

    let ast = parse(src);

    if args.verbosity == 1 {
        pretty_print(&ast, 0);
    }

    let json_string = serde_json::to_string(&ast).unwrap();
    match File::create(&args.output) {
        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
            Ok(_) => println!("writing: {}", args.output),
            Err(error) => println!("...failed to write {}:{}", args.output, error),
        },
        Err(error) => println!("...failed to open {}:{}", args.output, error),
    }
}

fn pretty_print(_ast: &Vec<Node>, depth: usize) {
    for n in _ast.into_iter() {
        println!("{}tag: {}", SEPARATOR.repeat(depth), n.tag.value());
        println!("{}value: {}", SEPARATOR.repeat(depth), n.value);
        println!("{}children: {}", SEPARATOR.repeat(depth), n.children.len());
        pretty_print(&n.children, depth + 1);
    }
}

fn parse(src: String) -> Vec<Node> {
    let mut ast = Vec::<Node>::new();

    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            let mut n = Node {
                children: Vec::<Node>::new(),
                tag: Box::new(Environment::Root),
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

            ast
        }
        Err(error) => {
            println!("error parsing: {}", error);
            ast
        }
    }
}

fn parse_section(_section: Pair<Rule>) -> Node {
    let mut section_node = Node {
        children: Vec::<Node>::new(),
        tag: Box::new(Environment::Paragraph),
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
                    tag: Box::new(Token::Literal),
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
        tag: Box::new(Environment::Paragraph), //-- todo: change this to empty box?
        value: String::from(""),
    };

    for subpair in _env.into_inner() {
        match subpair.as_rule() {
            Rule::env_name => match environments::parse_name(subpair.as_str()) {
                Some(env) => env_node.tag = Box::new(env),
                None => panic!("Could not parse environment name: {}", subpair.as_str()),
            },
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
            Rule::code_description => {
                env_node.value = String::from(subpair.as_str())
            }
            Rule::env_stmt_opts => {
                env_node.value = format!("{}-{}", env_node.value, subpair.as_str())
            }
            _ => println!("-- unexpected environment {:?}", subpair.as_span()),
        }
    }

    env_node
}

//-- parse_command can return None if the parsed Node is only related print layout
fn parse_command(_stmt: Pair<Rule>) -> Option<Node> {
    let mut cmd_node = Node {
        children: Vec::<Node>::new(),
        tag: Box::new(Token::Command),
        value: String::from(""),
    };

    for subpair in _stmt.into_inner() {
        match subpair.as_rule() {
            Rule::ctrl_character => (),
            Rule::name => match commands::parse_name(subpair.as_str()) {
                Some(cmd) => {
                    if cmd.is_print_layout() {
                        return None;
                    } else {
                        cmd_node.tag = Box::new(cmd)
                    }
                }
                None => panic!("Could not parse command: {}", subpair.as_str()),
            },
            Rule::cmd_stmt_opts => {
                cmd_node.value = String::from(subpair.as_str())
            }
            Rule::cmd_stmt => match parse_command(subpair) {
                Some(n) => cmd_node.children.push(n),
                None => panic!("Could not parse nested command:"),
            },
            Rule::literal_group => {
                cmd_node.children.push(Node {
                    tag: Box::new(Token::Literal),
                    value: String::from(subpair.as_str()),
                    children: Vec::<Node>::new(),
                });
            }
            _ => println!("unexpected: {:?}", subpair.as_rule()),
        }
    }

    Some(cmd_node)
}

#[test]
fn it_parses_a_file() {
    let test_src = fs::read_to_string("latex_test.tex").expect("Cannot open file");
    let test_ast = parse(test_src);
    assert_eq!(test_ast.len(), 1);

    let top_level = test_ast.first().unwrap();
    assert_eq!("root", top_level.tag.value());
    assert_eq!(top_level.children.len(), 1);

    let document_environment = top_level
        .children
        .first()
        .unwrap()
        .children
        .first()
        .unwrap();
    assert_eq!("document", document_environment.tag.value());
    assert_eq!(3, document_environment.children.len());

    //-- check the first header
    let header_section = document_environment
        .children
        .first()
        .unwrap()
        .children
        .first()
        .unwrap();
    assert_eq!("section", header_section.tag.value());
    // assert_eq!("section", header_section.value);

    //-- check the listing environment
    let listing = document_environment
        .children
        .get(1)
        .unwrap()
        .children
        .get(0)
        .unwrap();
    assert_eq!("figure", listing.tag.value());
    // assert_eq!("figure", listing.value);

    //-- check the code environment
    let code = listing.children.first().unwrap().children.first().unwrap();
    assert_eq!("code", code.tag.value());
    assert_eq!("python", code.value);

    //-- check the caption
    let caption = listing.children.get(1).unwrap().children.first().unwrap();
    assert_eq!("caption", caption.tag.value());
    assert_eq!("[]", caption.value);

    //-- check the label
    let label = listing.children.get(2).unwrap().children.first().unwrap();
    assert_eq!("label", label.tag.value());
    // assert_eq!("label", label.value)
}
