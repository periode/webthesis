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
dyn_clone::clone_trait_object!(Tag);

#[derive(Serialize, Clone)]
pub struct Node {
    pub children: Option<Vec<Node>>,
    pub tag: Box<dyn Tag>,
    pub value: String,
    pub index: i32,
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
pub struct State {
    include: String,
    footnote_index: i32,
    paragraph_index: i32,
}

impl State {
    pub fn new() -> State {
        return State {
            include: String::from(""),
            footnote_index: 0,
            paragraph_index: 0,
        };
    }
    fn set_include(&mut self, _include: String) {
        self.include = _include
    }

    fn get_include(&self) -> String {
        return self.include.clone();
    }

    fn increment_paragraph(&mut self) {
        self.paragraph_index += 1;
    }

    fn get_paragraph(&mut self) -> i32 {
        return self.paragraph_index;
    }

    //-- to be called whenever a footnote is encountered
    //-- increases the counter by one and returns the new value
    fn register_footnote(&mut self) -> i32 {
        self.footnote_index += 1;
        self.footnote_index.clone()
    }
}

pub fn parse(src: String, state: &mut State) -> Vec<Node> {
    let mut ast = Vec::<Node>::new();

    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            for subpair in pair.into_inner() {
                match subpair.as_rule() {
                    Rule::paragraph => {
                        let mut s = parse_paragraph(subpair, state);
                        if let Some(_) = &s.children {
                            state.increment_paragraph();
                            s.index = state.get_paragraph();
                            ast.push(s);
                        }
                    }
                    Rule::env_stmt => {
                        let e = parse_environment(subpair, state);
                        ast.push(e);
                    }
                    Rule::cmd_stmt => {
                        if let Some(c) = parse_command(subpair, state) {
                            ast.push(c);
                        }
                    }
                    // Rule::COMMENT => println!("{:?} -{}", subpair.as_rule(), subpair.as_str()),
                    Rule::EOI => (),
                    _ => println!("UNKNOWN {:?}", subpair.as_rule()),
                }
            }

            ast
        }
        Err(error) => {
            println!("error parsing: {}", error);
            ast
        }
    }
}

fn parse_paragraph(_section: Pair<Rule>, state: &mut State) -> Node {
    let mut paragraph = Node {
        children: None,
        tag: Box::new(Environment::Paragraph),
        value: String::from(""),
        index: 0,
    };

    for subpair in _section.into_inner() {
        match subpair.as_rule() {
            Rule::cmd_stmt => {
                if let Some(c) = parse_command(subpair, state) {
                    paragraph.add(c)
                }
            }
            Rule::literal_group => {
                let l = Node {
                    tag: Box::new(Token::Literal),
                    value: String::from(subpair.as_str()),
                    children: None,
                    index: 0,
                };
                paragraph.add(l);
            }
            // Rule::COMMENT => println!("{}", subpair.as_str()),
            _ => println!("unable to parse paragraph:{:?}", subpair.as_rule()),
        }
    }

    paragraph
}

fn parse_environment(_env: Pair<Rule>, state: &mut State) -> Node {
    let mut env_node = Node {
        children: None,
        tag: Box::new(Environment::Paragraph), //-- todo: change this to empty box?
        value: String::from(""),
        index: 0,
    };

    let mut is_paragraph = false;
    for subpair in _env.into_inner() {
        match subpair.as_rule() {
            Rule::env_name => match environments::parse_name(subpair.as_str()) {
                Some(env) => {
                    env_node.tag = Box::new(env);
                    if env == Environment::Quote
                        || env == Environment::Itemize
                        || env == Environment::Listing
                        || env == Environment::Figure
                    {
                        is_paragraph = true;
                    }
                }
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
                        Rule::code_stmt => {
                            let e = parse_environment(subsubpair, state);
                            env_node.add(e);
                        }
                        Rule::env_stmt => {
                            let e = parse_environment(subsubpair, state);
                            env_node.add(e);
                        }
                        Rule::cmd_stmt => {
                            if let Some(c) = parse_command(subsubpair, state) {
                                if c.tag.value() == Command::Label.value() {
                                    //-- wtf: assign the label value directly inside the environment
                                    let tmp = c.clone().to_owned().children.unwrap();
                                    let first = tmp.first().unwrap();
                                    env_node.value = first.value.clone();
                                }

                                env_node.add(c);
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
                    index: 0,
                };

                env_node.add(l);
            }
            Rule::opts => {
                let opts = subpair.into_inner().next().unwrap();
                let o = Node {
                    children: None,
                    tag: Box::new(Token::Options),
                    value: String::from(opts.as_str()),
                    index: 0,
                };

                env_node.add(o);
            }
            _ => println!("-- unexpected environment {:?}", subpair.as_span()),
        }
    }

    if is_paragraph {
        // hmmm... the quote is semantically a paragraph, but technically an environment
        state.increment_paragraph();
        let mut p = Node {
            children: None,
            tag: Box::new(Environment::Paragraph),
            value: String::from(""),
            index: state.get_paragraph(),
        };

        p.add(env_node);

        return p;
    }
    env_node
}

//-- parse_command can return None if the parsed Node is only related print layout
fn parse_command(_stmt: Pair<Rule>, state: &mut State) -> Option<Node> {
    let mut cmd_node = Node {
        children: None,
        tag: Box::new(Token::Command),
        value: String::from(""),
        index: 0,
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
                let children = parse(src, state);
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
                        } else if cmd == Command::Citation {
                            cmd_node.index = state.get_paragraph()
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
                    index: 0,
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
                    index: 0,
                };

                cmd_node.add(l);
            }
            _ => println!("unexpected: {:?}", subpair.as_rule()),
        }
    }

    Some(cmd_node)
}

pub fn save(nodes: Vec<Node>, dest: &str) {
    let json_string = serde_json::to_string(&nodes).unwrap();
    match File::create(format!("{}/full.json", &dest)) {
        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
            Ok(_) => println!("- writing: {}/full.json ", dest),
            Err(error) => println!("...failed to write {}:{}", dest, error),
        },
        Err(error) => println!("...failed to open {}:{}", dest, error),
    }

    print!("- writing: ");
    for document in nodes.into_iter() {
        let mut front: Vec<Node> = Vec::<Node>::new();
        for node in document.children.unwrap().into_iter() {
            if node.tag.value() == Command::Include.value() {
                let json_string = serde_json::to_string(&node).unwrap();
                let fname = node.value.split(".").next().unwrap();
                match File::create(format!("{}/{}.json", &dest, fname)) {
                    Ok(mut output_file) => match write!(output_file, "{}", json_string) {
                        Ok(_) => print!("{}/{}.json ", dest, fname),
                        Err(error) => {
                            println!("...failed to write {}:{}", dest, error)
                        }
                    },
                    Err(error) => println!("...failed to open {}:{}", dest, error),
                }
            } else if node.tag.is_front() {
                front.push(node);
            }
        }

        if front.len() > 0 {
            let json_string = serde_json::to_string(&front).unwrap();
            let fname = "front";
            match File::create(format!("{}/{}.json", &dest, fname)) {
                Ok(mut output_file) => match write!(output_file, "{}", json_string) {
                    Ok(_) => print!("{}/{}.json", dest, fname),
                    Err(error) => println!("...failed to write {}:{}", dest, error),
                },
                Err(error) => println!("...failed to open {}:{}", dest, error),
            }
        }
        println!()
    }
}
