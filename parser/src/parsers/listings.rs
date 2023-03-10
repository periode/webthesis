extern crate pest;
use clap::Parser as ArgParser;
use pest::iterators::Pair;
use pest::Parser;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::foliage::commands;
use crate::foliage::commands::Command;
use crate::foliage::environments::{self, Environment};
use crate::Args;

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

#[derive(Serialize, Debug, Clone)]
pub struct ListingNode {
    index: usize,
    filename: String,
    label: String,
    value: String,
    section: String,
    section_label: String,
    chapter: String,
    chapter_label: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct State {
    index: usize,
    chapter: String,
    chapter_label: String,
    section: String,
    section_label: String,
}

impl State {
    pub fn new() -> State {
        return State {
            index: 0,
            section: String::from(""),
            section_label: String::from(""),
            chapter: String::from(""),
            chapter_label: String::from(""),
        };
    }
    fn increment_index(&mut self) {
        self.index += 1
    }

    fn get_index(&self) -> usize {
        self.index.clone()
    }

    fn set_chapter(&mut self, _chapter: String) {
        self.chapter = _chapter
    }

    fn get_chapter(&self) -> String {
        self.chapter.clone()
    }

    fn set_chapter_label(&mut self, _chapter_label: String) {
        self.chapter_label = _chapter_label
    }

    fn get_chapter_label(&self) -> String {
        self.chapter_label.clone()
    }

    fn set_section(&mut self, _section: String) {
        self.section = _section
    }

    fn get_section(&self) -> String {
        self.section.clone()
    }

    fn set_section_label(&mut self, _section_label: String) {
        self.section_label = _section_label
    }

    fn get_section_label(&self) -> String {
        self.section_label.clone()
    }
}

pub fn parse(src: String, state: &mut State) -> Vec<ListingNode> {

    let mut list = Vec::<ListingNode>::new();

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
                            if let Some(mut listing) = parse_env(subpair, state) {
                                list.append(&mut listing);
                            }
                        }
                        Rule::cmd_stmt => {
                            if let Some(mut n) = parse_command(subpair, state) {
                                list.append(&mut n);
                            }
                        }
                        _ => (),
                    },
                    None => break,
                }
            }
        }
        Err(error) => {
            println!("error parsing: {}", error);
        }
    }

    list
}

fn parse_env(_env: Pair<Rule>, _state: &mut State) -> Option<Vec<ListingNode>> {
    let mut pair_iter = _env.into_inner();

    let mut nodes = Vec::<ListingNode>::new();
    loop {
        match pair_iter.next() {
            Some(pair) => match pair.as_rule() {
                Rule::paragraph => {
                    if let Some(mut n) = parse_env(pair, _state) {
                        nodes.append(&mut n);
                    }
                }
                Rule::cmd_stmt => {
                    if let Some(mut n) = parse_command(pair, _state) {
                        nodes.append(&mut n);
                    }
                }
                Rule::env_name => {
                    let mut env_iter = pair.into_inner();
                    let env_name = env_iter.next().unwrap();

                    match environments::parse_name(env_name.as_str()) {
                        Some(name) => {
                            if name.is_listing() {
                                let env_content = pair_iter.next().unwrap();
                                let mut contents = env_content.into_inner();

                                //-- skipping the \include
                                contents.next().unwrap();

                                let include_cmd = contents.next().unwrap();
                                let mut include = include_cmd.into_inner();
                                include.next().unwrap();

                                if name == Environment::Figure {
                                    include.next().unwrap();
                                }
                                let include_literal = include.next().unwrap().as_str();

                                let caption_cmd = contents.next().unwrap();
                                let mut caption = caption_cmd.into_inner();
                                caption.next().unwrap();
                                let caption_literal = caption.next().unwrap().as_str();

                                let label_cmd = contents.next().unwrap();
                                let mut label = label_cmd.into_inner();
                                label.next().unwrap();
                                let label_literal = label.next().unwrap().as_str();

                                _state.increment_index();

                                let l = ListingNode {
                                    index: _state.get_index(),
                                    filename: include_literal.to_string(),
                                    label: label_literal.to_string(),
                                    value: caption_literal.to_string(),
                                    section: _state.get_section(),
                                    section_label: _state.get_section_label(),
                                    chapter: _state.get_chapter(),
                                    chapter_label: _state.get_chapter_label(),
                                };

                                nodes.push(l);
                            } else {
                                let env_content = pair_iter.next().unwrap();
                                if let Some(mut n) = parse_env(env_content, _state) {
                                    nodes.append(&mut n);
                                }
                            }
                        }
                        None => (),
                    }
                }
                _ => (),
            },
            None => break,
        }
    }

    Some(nodes)
}

fn parse_command(_cmd: Pair<Rule>, _state: &mut State) -> Option<Vec<ListingNode>> {
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
                let children = parse(src, _state);

                return Some(children);
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
                        match cmd {
                            Command::Chapter => {
                                let p = pair_iter.next().unwrap();
                                _state.set_chapter(p.as_str().to_string());
                            }
                            Command::Section => {
                                let p = pair_iter.next().unwrap();
                                _state.set_section(p.as_str().to_string());
                            }
                            Command::Label => {
                                let p = pair_iter.next().unwrap();
                                let label = p.as_str().split(":").collect::<Vec<&str>>();
                                match label[0] {
                                    "chap" => _state.set_chapter_label(label[1].to_string()),
                                    "sec" => _state.set_section_label(label[1].to_string()),
                                    _ => (),
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

    None
}

pub fn save(nodes: Vec<ListingNode>, dest: &str) {
    let json_string = serde_json::to_string(&nodes).unwrap();
    match File::create(format!("{}/listings.json", &dest)) {
        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
            Ok(_) => println!("- writing: {}/listings.json", dest),
            Err(error) => println!("...failed to write {}:{}", dest, error),
        },
        Err(error) => println!("...failed to open {}:{}", dest, error),
    }
}
