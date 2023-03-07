use chrono::Utc;
use serde::Serialize;

extern crate pest;
use clap::Parser as ArgParser;
use pest::iterators::Pair;
use pest::Parser;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::foliage::commands::Command;
use crate::foliage::environments::{self, Environment};
use crate::foliage::tokens::Token;
use crate::foliage::{commands, Tag};
use crate::Args;

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

#[derive(Serialize)]
pub struct Node {
    pub children: Option<Vec<Node>>,
    pub tag: Box<dyn Tag>,
    pub value: String,
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

#[derive(Clone)]
struct State {
    include: String,
    footnote_index: i32,
}

impl State {
    fn set_include(&mut self, _include: String) {
        self.include = _include
    }

    fn get_include(&self) -> String {
        self.include.clone()
    }

    //-- to be called whenever a footnote is encountered
    //-- increases the counter by one and returns the new value
    fn register_footnote(&mut self) -> i32 {
        self.footnote_index += 1;
        self.footnote_index.clone()
    }
}

pub fn parse(src: String) -> Vec<Node> {
    let mut state = State {
        include: String::from(""),
        footnote_index: 0,
    };

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
                        let s = parse_paragraph(subpair, &mut state);
                        if let Some(_) = &s.children {
                            n.add(s);
                        }
                    }
                    Rule::env_stmt => {
                        let e = parse_environment(subpair, &mut state);
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

fn parse_paragraph(_section: Pair<Rule>, state: &mut State) -> Node {
    let mut section_node = Node {
        children: None,
        tag: Box::new(Environment::Paragraph),
        value: String::from(""),
    };

    for subpair in _section.into_inner() {
        match subpair.as_rule() {
            Rule::env_stmt => {
                let e = parse_environment(subpair, state);
                section_node.add(e);
            }
            Rule::code_stmt => {
                let e = parse_environment(subpair, state);
                section_node.add(e);
            }
            Rule::cmd_stmt => {
                if let Some(c) = parse_command(subpair, state) {
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
                let s = parse_paragraph(subpair, state);
                if let Some(_) = &s.children {
                    //-- skip empty paragraphs
                    section_node.add(s);
                }
            }
            Rule::COMMENT => println!("{}", subpair.as_str()),
            _ => println!("unable to parse paragraph:{:?}", subpair.as_rule()),
        }
    }

    section_node
}

fn parse_environment(_env: Pair<Rule>, state: &mut State) -> Node {
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
                            let s = parse_paragraph(subsubpair, state);
                            if let Some(_) = &s.children {
                                //-- skip empty paragraphs
                                env_node.add(s);
                            }
                        }
                        _ => println!(
                            "could not parse inside environment: {:?}",
                            subsubpair.as_rule()
                        ),
                    }
                }
            }
            Rule::language => {
                env_node.tag = Box::new(Environment::Minted);
                env_node.value = String::from(subpair.as_str());
            }
            Rule::filepath => {
                let l = Node {
                    tag: Box::new(Token::Literal),
                    value: String::from(subpair.as_str()),
                    children: None,
                };

                env_node.add(l);
            }
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
fn parse_command(_stmt: Pair<Rule>, state: &mut State) -> Option<Node> {
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

                let i = include.file_stem().unwrap().to_str().unwrap();
                state.set_include(String::from(i));

                let src = fs::read_to_string(fp).expect("Cannot open file");
                let children = parse(src);
                cmd_node.tag = Box::new(Command::Include);
                cmd_node.value = include.display().to_string();
                for c in children {
                    cmd_node.add(c);
                }

                return Some(cmd_node);
            }
            Command::Date => cmd_node.value = Utc::now().timestamp().to_string(),
            _ => (),
        }
    }

    for subpair in _stmt.into_inner() {
        match subpair.as_rule() {
            Rule::ctrl_character => (),
            Rule::name => match commands::parse_name(subpair.as_str()) {
                Some(cmd) => {
                    if cmd.is_semantic() {
                        cmd_node.tag = Box::new(cmd);

                        if cmd == Command::Label {
                            let i = state.get_include();
                            cmd_node.value = i;
                        } else if cmd == Command::Footnote {
                            let i = state.register_footnote();
                            cmd_node.value = i.to_string();
                        }
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
            Rule::cmd_stmt => match parse_command(subpair, state) {
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

pub fn save(nodes: Vec<Node>, dest: &str, split: bool) {
    if split {
        print!("writing: ");
        for first in nodes.into_iter() {
            for second in first.children.unwrap().into_iter() {
                for third in second.children.unwrap().into_iter() {
                    let mut front: Vec<Node> = Vec::<Node>::new();
                    for fourth in third.children.unwrap().into_iter() {
                        for fifth in fourth.children.unwrap().into_iter() {
                            if fifth.tag.value() == Command::Include.value() {
                                let json_string = serde_json::to_string(&fifth).unwrap();
                                let fname = fifth.value.split(".").next().unwrap();
                                match File::create(format!("{}/{}.json", &dest, fname)) {
                                    Ok(mut output_file) => {
                                        match write!(output_file, "{}", json_string) {
                                            Ok(_) => print!("{}/{}.json ", dest, fname),
                                            Err(error) => {
                                                println!("...failed to write {}:{}", dest, error)
                                            }
                                        }
                                    }
                                    Err(error) => println!("...failed to open {}:{}", dest, error),
                                }
                            } else if fifth.tag.is_front() {
                                front.push(fifth);
                            }
                        }
                    }

                    let json_string = serde_json::to_string(&front).unwrap();
                    let fname = "front";
                    match File::create(format!("{}/{}.json", &dest, fname)) {
                        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
                            Ok(_) => println!("writing: {}/{}.json", dest, fname),
                            Err(error) => println!("...failed to write {}:{}", dest, error),
                        },
                        Err(error) => println!("...failed to open {}:{}", dest, error),
                    }
                }
            }
        }
    } else {
        let json_string = serde_json::to_string(&nodes).unwrap();
        match File::create(format!("{}/text.json", &dest)) {
            Ok(mut output_file) => match write!(output_file, "{}", json_string) {
                Ok(_) => println!("writing: {}/text.json", dest),
                Err(error) => println!("...failed to write {}:{}", dest, error),
            },
            Err(error) => println!("...failed to open {}:{}", dest, error),
        }
    }
}
