pub mod foliage;

use std::fs::{self, File};
use std::io::Write;
use std::iter::Peekable;
use std::path::Path;
use std::slice::Iter;
use std::time::Instant;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use chrono::Utc;
use clap::Parser as ArgParser;
use foliage::commands::Command;
use foliage::environments::Environment;
use foliage::tokens::Token;
use foliage::{commands, environments, Tag};
use pest::iterators::Pairs;
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

#[derive(Serialize, Clone, Debug)]
struct ToCNode {
    children: Option<Vec<ToCNode>>,
    tag: Command,
    value: String,
    label: String,
    indent: i8,
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

const SEPARATOR: &str = " | ";
const DEFAULT_INPUT: &str = "./test_inputs/include.tex";
const DEFAULT_OUTPUT_DIR: &str = "output";
const DEFAULT_VERBOSE: usize = 0;
#[derive(ArgParser, Debug)]
struct Args {
    #[arg(short, default_value = DEFAULT_INPUT)]
    input: String,

    #[arg(short, default_value = DEFAULT_OUTPUT_DIR)]
    output_dir: String,

    #[arg(short, default_value_t = DEFAULT_VERBOSE)]
    verbosity: usize,
}

fn main() {
    let args = Args::parse();
    let input = args.input.to_string();
    let fp = Path::new(&input);
    let src = fs::read_to_string(fp.as_os_str()).expect("Cannot open file");
    println!("reading: {}", args.input);

    let mut state = State {
        include: String::from(""),
        footnote_index: 0,
    };

    let start = Instant::now();
    println!("parsing text...");
    let ast = parse(src.clone(), &mut state);
    println!("parsing table of contents...");
    let toc = parse_toc(src.clone());
    let duration = start.elapsed();

    if args.verbosity == 1 {
        pretty_print(&ast, 0);
    }

    save_ast(ast, &args.output_dir);
    save_toc(toc, &args.output_dir);
    println!("lasting: {:?}", duration)
}

fn save_ast(nodes: Vec<Node>, dest: &str) {
    let json_string = serde_json::to_string(&nodes).unwrap();
    match File::create(format!("{}/text.json", &dest)) {
        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
            Ok(_) => println!("writing: {}/text.json", dest),
            Err(error) => println!("...failed to write {}:{}", dest, error),
        },
        Err(error) => println!("...failed to open {}:{}", dest, error),
    }
}

fn save_toc(nodes: Vec<ToCNode>, dest: &str) {
    let json_string = serde_json::to_string(&nodes).unwrap();
    match File::create(format!("{}/toc.json", &dest)) {
        Ok(mut output_file) => match write!(output_file, "{}", json_string) {
            Ok(_) => println!("writing: {}/toc.json", dest),
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

fn parse_toc(src: String) -> Vec<ToCNode> {
    let mut toc = Vec::<ToCNode>::new();
    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            let start_node = pair;

            // println!("the start node is {:?} {}", start_node.as_rule(), start_node.as_str());

            let mut start_iter = start_node.into_inner();
            loop {
                let pair = start_iter.next();
                match pair {
                    Some(subpair) => {
                        // println!("the current node is {:?} {}", subpair.as_rule(), subpair.as_str());
                        match subpair.as_rule() {
                            Rule::env_content => {
                                //-- if we're at the root
                                println!("parsing env: {}", subpair.as_str());
                                if let Some(n) = parse_paragraph_toc(subpair) {
                                    toc.push(n);
                                }
                            }
                            Rule::paragraph => {
                                
                                let s = parse_paragraph_toc(subpair);
                                if let Some(mut node) = s {
                                    node.label = parse_label(&mut start_iter);
                                    toc.push(node.clone());
                                }
                            }
                            _ => (),
                        }
                    }
                    None => break,
                }
            }

            let mut toc_iter = toc.iter().peekable();
            if let Some(start) = toc_iter.next() {
                let mut sc = start.clone();
                let final_toc = make_tree(&mut sc, &mut toc_iter, 0);

                return final_toc;
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

fn make_tree(
    current: &mut ToCNode,
    iter: &mut Peekable<Iter<ToCNode>>,
    indent: u8,
) -> Vec<ToCNode> {
    let mut tree = Vec::<ToCNode>::new();

    loop {
        if let Some(next) = iter.peek() {
            match assess_toc_relation(&next, &current.tag) {
                0 => {
                    //-- find siblings
                    println!(
                        "[{}] found siblings: {} {}",
                        indent,
                        next.tag.value(),
                        next.value
                    );
                    tree.push(iter.next().unwrap().clone());
                }
                1 => {
                    println!(
                        "[{}] found {} as child of {}",
                        indent,
                        next.tag.value(),
                        current.tag.value()
                    );

                    let mut c = iter.next().unwrap().clone();
                    let grandchildren = make_tree(&mut c, iter, indent + 1);

                    if grandchildren.len() == 0 {
                        tree.push(current.clone());
                        return tree;
                    } else {
                        current.children = Some(grandchildren);
                    }
                }
                _ => {
                    println!(
                        "[{}] found different one: {} {}, should go up",
                        indent,
                        next.tag.value(),
                        next.value
                    );

                    let c = current.clone();
                    tree.push(c);
                    return tree;
                }
            }
        } else {
            println!(
                "[{}] adding {} {}",
                indent,
                current.tag.value(),
                current.value
            );
            tree.push(current.clone());
            return tree;
        }
    }
}

fn assess_toc_relation(next: &ToCNode, current: &Command) -> i8 {
    return current.get_indent() - next.tag.get_indent();
}

//-- this finds a label on the following line of the given _pairs
fn parse_label(_pairs: &mut Pairs<Rule>) -> String {
    if let Some(par) = _pairs.next() {
        //-- next paragraph
        let mut par_iter = par.into_inner();
        if let Some(label_cmd) = par_iter.next() {
            //-- actual label cmd
            let mut label = label_cmd.into_inner();
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
        } else {
            println!("[WARN] [no-label] no command found after header");
        }
    }
    String::from("")
}

//-- this one is just for recursion (either paragraph, and keep on going, or command, and check for headers)
fn parse_paragraph_toc(_paragraph: Pair<Rule>) -> Option<ToCNode> {
    let mut pair_iter = _paragraph.into_inner();
    let mut paragraph = ToCNode{
        tag: Command::Documentclass,
        value: String::from(""),
        label: String::from(""),
        children: None,
        indent: 0,
    };

    loop {
        match pair_iter.next() {
            Some(pair) => match pair.as_rule() {
                Rule::env_stmt => {
                    let mut env_iter = pair.into_inner();
                    let _env_name = env_iter.next();
                    let env_content = env_iter.next().unwrap();

                    if let Some(n) = parse_paragraph_toc(env_content) {
                        return Some(n);
                    }
                }
                Rule::paragraph => {
                    if let Some(node) = parse_paragraph_toc(pair) {
                        paragraph.add(node);
                    }
                }
                Rule::cmd_stmt => return parse_command_toc(pair),
                _ => (),
            },
            None => break,
        }
    }

    if let Some(_) = paragraph.children {
        return Some(paragraph);
    }else{
        return None;
    }
    
}

fn parse_command_toc(_cmd: Pair<Rule>) -> Option<ToCNode> {
    //-- check if we're currently at an \include
    // println!("parsing command: {}", _cmd.as_str());
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

                println!("including in toc: {:?}", fp);

                let src = fs::read_to_string(fp).expect("Cannot open file");
                let children = parse_toc(src);

                let n = ToCNode {
                    children: Some(children),
                    tag: Command::Include,
                    value: include.display().to_string(),
                    label: String::from(""),
                    indent: 0,
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
                                indent: 0,
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

fn parse(src: String, state: &mut State) -> Vec<Node> {
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
                        let s = parse_paragraph(subpair, state);
                        if let Some(_) = &s.children {
                            n.add(s);
                        }
                    }
                    Rule::env_stmt => {
                        let e = parse_environment(subpair, state);
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

                println!("including: {:?}", fp);

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

#[test]
fn it_parses_a_file() {
    let test_src = fs::read_to_string("test_inputs/basic.tex").expect("Cannot open file");
    let mut test_state = State {
        include: String::from(""),
        footnote_index: 0,
    };
    let test_ast = parse(test_src, &mut test_state);
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
