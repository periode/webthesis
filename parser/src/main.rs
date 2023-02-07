pub mod foliage;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use clap::Parser as ArgParser;
use foliage::commands::Command;
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
    children: Option<Vec<Node>>,
    tag: Box<dyn Tag>,
    value: String,
}

impl Node {
    pub fn add(&mut self, child: Node) {
        if let Some(_) = &self.children {
            self.children.as_mut().unwrap().push(child)
        } else {
            let mut v = Vec::<Node>::new();
            v.push(child);
            self.children = Some(v);
        }
    }
}

const SEPARATOR: &str = " | ";
const DEFAULT_INPUT: &str = "./test_inputs/basic.tex";
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
    let input = args.input.to_string();
    let fp = Path::new(&input);
    let src = fs::read_to_string(fp.as_os_str()).expect("Cannot open file");
    println!("reading: {}", args.input);

    let ast = parse(src);

    if args.verbosity == 1 {
        pretty_print(&ast, 0);
    }

    save_ast(ast, &args.output)
}

fn save_ast(nodes: Vec<Node>, dest: &str) {
    let json_string = serde_json::to_string(&nodes).unwrap();
    match File::create(&dest) {
        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
            Ok(_) => println!("writing: {}", dest),
            Err(error) => println!("...failed to write {}:{}", dest, error),
        },
        Err(error) => println!("...failed to open {}:{}", dest, error),
    }
}

fn pretty_print(_ast: &Vec<Node>, depth: usize) {
    for n in _ast.into_iter() {
        println!("{}tag: {}", SEPARATOR.repeat(depth), n.tag.value());
        println!("{}value: {}", SEPARATOR.repeat(depth), n.value);
        match &n.children {
            Some(c) => {
                println!("{}children: {}", SEPARATOR.repeat(depth), c.len());
                pretty_print(&c, depth + 1);
            }
            None => (),
        }
    }
}

fn parse(src: String) -> Vec<Node> {
    let mut ast = Vec::<Node>::new();

    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            let mut n = Node {
                children: Some(Vec::<Node>::new()),
                tag: Box::new(Environment::Root),
                value: String::from(""),
            };

            for subpair in pair.into_inner() {
                match subpair.as_rule() {
                    Rule::paragraph => {
                        let s = parse_paragraph(subpair);
                        n.add(s);
                    }
                    Rule::env_stmt => {
                        let e = parse_environment(subpair);
                        n.add(e);
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

fn parse_paragraph(_section: Pair<Rule>) -> Node {
    let mut section_node = Node {
        children: None,
        tag: Box::new(Environment::Paragraph),
        value: String::from(""),
    };

    for subpair in _section.into_inner() {
        match subpair.as_rule() {
            Rule::env_stmt => {
                let e = parse_environment(subpair);
                section_node.add(e);
            }
            Rule::code_stmt => {
                let e = parse_environment(subpair);
                section_node.add(e);
            }
            Rule::cmd_stmt => {
                if let Some(c) = parse_command(subpair) {
                    section_node.add(c);
                }
            }
            Rule::literal_group => {
                let l = Node {
                    tag: Box::new(Token::Literal),
                    value: String::from(subpair.as_str()),
                    children: None,
                };
                section_node.add(l);
            }
            Rule::paragraph => {
                let s = parse_paragraph(subpair);
                if let Some(_) = &s.children {
                    //-- skip empty sections
                    section_node.add(s);
                }
            }
            Rule::COMMENT => println!("{}", subpair.as_str()),
            _ => println!("unable to parse paragraph:{:?}", subpair.as_rule()),
        }
    }

    section_node
}

fn parse_environment(_env: Pair<Rule>) -> Node {
    let mut env_node = Node {
        children: None,
        tag: Box::new(Environment::Paragraph), //-- todo: change this to empty box?
        value: String::from(""),
    };

    for subpair in _env.into_inner() {
        match subpair.as_rule() {
            Rule::env_name => match environments::parse_name(subpair.as_str()) {
                Some(env) => env_node.tag = Box::new(env),
                None => println!("Could not parse environment name: {}", subpair.as_str()),
            },
            Rule::env_content => {
                for subsubpair in subpair.into_inner() {
                    match subsubpair.as_rule() {
                        Rule::paragraph => {
                            let s = parse_paragraph(subsubpair);
                            if let Some(_) = &s.children {
                                //-- skip empty sections
                                env_node.add(s);
                            }
                        }
                        _ => println!("could not parse inside environment: {:?}", subsubpair.as_rule()),
                    }
                }
            }
            Rule::code_content => {
                env_node.tag = Box::new(Environment::Minted);

                for subsubpair in subpair.into_inner() {
                    match subsubpair.as_rule() {
                        Rule::paragraph => {
                            let s = parse_paragraph(subsubpair);
                            if let Some(_) = &s.children {
                                //-- skip empty sections
                                env_node.add(s);
                            }
                        }
                        Rule::literal_group_code => {
                            let l = Node {
                                tag: Box::new(Token::Literal),
                                value: String::from(subsubpair.as_str()),
                                children: None,
                            };
            
                            env_node.add(l);
                        }
                        _ => println!("could not parse inside code: {:?}", subsubpair.as_rule()),
                    }
                }
            }
            Rule::code_description => env_node.value = String::from(subpair.as_str()),
            Rule::opts => {
                let opts = subpair.into_inner().next().unwrap();
                let o = Node {
                    children: None,
                    tag: Box::new(Token::Options),
                    value: String::from(opts.as_str()),
                };

                env_node.add(o);
            }
            _ => println!("-- unexpected environment {:?}", subpair.as_span()),
        }
    }

    env_node
}

//-- parse_command can return None if the parsed Node is only related print layout
fn parse_command(_stmt: Pair<Rule>) -> Option<Node> {
    let mut cmd_node = Node {
        children: None,
        tag: Box::new(Token::Command),
        value: String::from(""),
    };

    //-- check if we're currently at an \include
    let mut s = _stmt.clone().into_inner();
    if let Some(c) = commands::parse_name(s.next().unwrap().as_str()) {
        match c {
            Command::Include => {
                let include = Path::new(s.next().unwrap().as_str());
                let fp;
                if include.is_absolute() {
                    fp = include.display().to_string();
                } else {
                    let args = Args::parse();
                    let input = args.input.to_string();
                    let root = Path::new(&input);

                    fp = format!("{}/{}", root.parent().unwrap().display(), include.display());
                }

                println!("including: {:?}", fp);

                let src = fs::read_to_string(fp).expect("Cannot open file");
                let children = parse(src);
                cmd_node.tag = Box::new(Command::Include);
                for c in children {
                    cmd_node.add(c);
                }

                return Some(cmd_node);
            }
            _ => (),
        }
    }

    for subpair in _stmt.into_inner() {
        match subpair.as_rule() {
            Rule::ctrl_character => (),
            Rule::name => match commands::parse_name(subpair.as_str()) {
                Some(cmd) => {
                    if cmd.is_semantic() {
                        cmd_node.tag = Box::new(cmd)
                    } else {
                        return None;
                    }
                }
                None => println!("Could not parse command: {}", subpair.as_str()),
            },
            Rule::opts => {
                let opts = subpair.into_inner().next().unwrap();
                let o = Node {
                    tag: Box::new(Token::Options),
                    value: String::from(opts.as_str()),
                    children: None,
                };

                cmd_node.add(o);
            }
            Rule::cmd_stmt => match parse_command(subpair) {
                Some(n) => cmd_node.add(n),
                None => println!("Could not parse nested command:"),
            },
            Rule::literal_group => {
                let l = Node {
                    tag: Box::new(Token::Literal),
                    value: String::from(subpair.as_str()),
                    children: None,
                };

                cmd_node.add(l);
            }
            _ => println!("unexpected: {:?}", subpair.as_rule()),
        }
    }

    Some(cmd_node)
}

#[test]
fn it_parses_a_file() {
    let test_src = fs::read_to_string("test_inputs/basic.tex").expect("Cannot open file");
    let test_ast = parse(test_src);
    assert_eq!(test_ast.len(), 1);

    let top_level = test_ast.first().unwrap();
    assert_eq!("root", top_level.tag.value());
    let ch = top_level.children.as_ref();
    assert_eq!(ch.unwrap().len(), 1);

    let document_environment = top_level
        .children
        .as_ref()
        .unwrap()
        .first()
        .unwrap()
        .children
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!("document", document_environment.tag.value());
    assert_eq!(4, document_environment.children.as_ref().unwrap().len());

    //-- check the first header
    let header_section = document_environment
        .children
        .as_ref()
        .unwrap()
        .first()
        .unwrap()
        .children
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!("section", header_section.tag.value());
    // assert_eq!("section", header_section.value);

    //-- check the listing environment
    let listing = document_environment
        .children
        .as_ref()
        .unwrap()
        .get(2)
        .unwrap()
        .children
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap();
    assert_eq!("listing", listing.tag.value());
    // assert_eq!("figure", listing.value);

    //-- check the code environment
    let code = listing
        .children
        .as_ref()
        .unwrap()
        .get(0)
        .unwrap()
        .children
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!("code", code.tag.value());
    assert_eq!("python", code.value);

    let code_opts = code.children.as_ref().unwrap().first().unwrap();
    assert_eq!("linenos,text=\\footnotesize", code_opts.value);

    //-- check the caption
    let caption = listing
        .children
        .as_ref()
        .unwrap()
        .get(1)
        .unwrap()
        .children
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!("caption", caption.tag.value());
    let caption_opts = caption.children.as_ref().unwrap().first().unwrap();
    assert_eq!("Short version", caption_opts.value);

    //-- check the label
    let label = listing
        .children
        .as_ref()
        .unwrap()
        .get(2)
        .unwrap()
        .children
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!("label", label.tag.value());
    let label_content = label.children.as_ref().unwrap().first().unwrap();
    assert_eq!("code:nielsen_chalktalk", label_content.value)
}
