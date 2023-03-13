extern crate pest;
use clap::Parser as ArgParser;
use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::iter::Peekable;
use std::path::Path;
use std::slice::Iter;

use crate::foliage::commands;
use crate::foliage::commands::Command;
use crate::Args;

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

#[derive(Serialize, Clone, Debug)]
pub struct ToCNode {
    children: Option<Vec<ToCNode>>,
    tag: Command,
    value: String,
    label: String,
    index: Vec<usize>,
}

impl ToCNode {
    pub fn add(&mut self, child: ToCNode) {
        if let Some(_) = &self.children {
            self.children.as_mut().unwrap().push(child)
        } else {
            let mut v = Vec::<ToCNode>::new();
            v.push(child);
            self.children = Some(v);
        }
    }

    pub fn set_index(&mut self, index: Vec<usize>) {
        self.index = index;
    }
}

pub fn parse(src: String) -> Vec<ToCNode> {
    let toc = make_toc_tree(src);

    let mut without_includes = Vec::<ToCNode>::new();
    for include in toc {
        if let Some(mut chap) = include.children {
            without_includes.append(&mut chap);
        }
    }
    
    let final_tree = prefix_toc(without_includes, &mut vec![]);
    return final_tree;
}

//-- this function parses the document, then assembles the AST
fn make_toc_tree(src: String) -> Vec<ToCNode> {
    let mut toc = Vec::<ToCNode>::new();

    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            let start_node = pair;

            let mut start_iter = start_node.into_inner();
            loop {
                let pair = start_iter.next();
                match pair {
                    Some(subpair) => match subpair.as_rule() {
                        Rule::env_stmt => {
                            if let Some(mut n) = parse_env(subpair) {
                                toc.append(&mut n);
                            }
                        }
                        Rule::cmd_stmt => {
                            if let Some(mut n) = parse_command(subpair) {
                                if n.tag.is_header() {
                                    let l = get_label(&mut start_iter);
                                    n.label = l;
                                }
                                toc.push(n);
                            }
                        }
                        _ => (),
                    },
                    None => break,
                }
            }

            //-- turn the list of toc headings into a tree
            let mut toc_iter = toc.iter().peekable();
            if let Some(s) = toc_iter.next() {
                let tree_toc = make_toc(&mut s.clone(), &mut toc_iter);

                return tree_toc;
            } else {
                return toc;
            }
        }
        Err(error) => {
            println!("error parsing: {}", error);
            toc
        }
    }
}

//-- this function populates the index member to represent the nested heading number
fn prefix_toc(_tree: Vec<ToCNode>, index: &mut Vec<usize>) -> Vec<ToCNode> {
    let mut tree = Vec::<ToCNode>::new();
    index.push(0);
    for mut node in _tree {
        if let Some(last) = index.last_mut() {
            *last += 1;

            node.index = index.clone();
            if let Some(ch) = node.children {
                let children = prefix_toc(ch, &mut index.clone());
                node.children = Some(children);
            }

            tree.push(node);
        }
    }
    return tree;
}

fn make_toc(
    current: &mut ToCNode,
    iter: &mut Peekable<Iter<ToCNode>>,
) -> Vec<ToCNode> {
    let mut tree = Vec::<ToCNode>::new();

    loop {
        if let Some(next) = iter.peek() {
            //-- we peek ahead before making a decision
            match assess_toc_relation(&next, &current.tag) {
                0 => {
                    let sibling = iter.next().unwrap().clone();
                    tree.push(sibling);
                }
                1 => {
                    let mut child = iter.next().unwrap().clone();

                    let grandchildren = make_toc(&mut child, iter);

                    //-- todo: demystify
                    if let Some(l) = tree.last_mut() {
                        //-- either get the last node on the tree
                        l.children = Some(grandchildren);
                    } else {
                        //-- if there is no last node, add to the current one
                        current.children = Some(grandchildren);
                    }
                }
                _ => {
                    let c = current.clone();
                    tree.insert(0, c);
                    return tree;
                }
            }
        } else {
            tree.insert(0, current.clone());
            return tree;
        }
    }
}

fn assess_toc_relation(next: &ToCNode, current: &Command) -> i8 {
    return current.get_indent() - next.tag.get_indent();
}

//-- this finds a label on the following line of the given _pairs
//-- used for the labelling of headings in the toc
fn get_label(_pairs: &mut Pairs<Rule>) -> String {
    if let Some(par) = _pairs.next() {
        let mut label = par.into_inner();
        if let Some(label_name) = label.next() {
            if let Some(cmd) = commands::parse_name(label_name.as_str()) {
                if cmd == Command::Label {
                    let label_content = label.next().unwrap();
                    return String::from(label_content.as_str());
                } else {
                    println!("[WARN] [no-label] following command is not a label!");
                }
            } else {
                println!("[WARN] [no-label] could not parse following command");
            }
        }
    }
    String::from("")
}

//-- parse env looks for commands in the environment content
fn parse_env(_env: Pair<Rule>) -> Option<Vec<ToCNode>> {
    let mut nodes = Vec::<ToCNode>::new();
    let mut pair_iter = _env.into_inner();
    loop {
        match pair_iter.next() {
            Some(pair) => match pair.as_rule() {
                Rule::env_content => {
                    for subpair in pair.into_inner() {
                        match subpair.as_rule() {
                            Rule::cmd_stmt => {
                                if let Some(n) = parse_command(subpair) {
                                    nodes.push(n);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            None => break,
        }
    }

    Some(nodes)
}

fn parse_command(_cmd: Pair<Rule>) -> Option<ToCNode> {
    //-- check if we're currently at an \include

    let mut s = _cmd.clone().into_inner();
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

                let src = fs::read_to_string(fp).expect("Cannot open file");
                let children = make_toc_tree(src);

                let n = ToCNode {
                    children: Some(children),
                    tag: Command::Include,
                    value: include.display().to_string(),
                    label: String::from(""),
                    index: vec![],
                };

                return Some(n);
            }
            _ => (),
        }
    }

    let mut pair_iter = _cmd.into_inner();
    loop {
        match pair_iter.next() {
            Some(subpair) => match subpair.as_rule() {
                Rule::name => {
                    if let Some(cmd) = commands::parse_name(subpair.as_str()) {
                        if cmd.is_header() {
                            let p = pair_iter.next().unwrap();
                            let n = ToCNode {
                                children: None,
                                tag: cmd,
                                value: String::from(p.as_str()),
                                index: vec![],
                                label: String::from(""),
                            };

                            return Some(n);
                        }
                    }
                }
                _ => (),
            },
            None => break,
        }
    }

    None
}

pub fn save(nodes: Vec<ToCNode>, dest: &str) {
    let json_string = serde_json::to_string(&nodes).unwrap();
    match File::create(format!("{}/toc.json", &dest)) {
        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
            Ok(_) => println!("- writing: {}/toc.json", dest),
            Err(error) => println!("...failed to write {}:{}", dest, error),
        },
        Err(error) => println!("...failed to open {}:{}", dest, error),
    }
}
