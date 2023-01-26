// from https://github.com/bign86/pest_latex

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

const SEPARATOR : &str = " | ";
fn main() {
    let input = "latex_test.tex";
    let src = fs::read_to_string(input).expect("Cannot open file");

    println!("parsing {}...", input);
    let indent = 0;
    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            for subpair in pair.into_inner() {
                println!("\n{:?}", subpair.as_rule());
                match subpair.as_rule() {
                    Rule::section => parse_section(subpair, indent),
                    Rule::env_stmt => parse_environment(subpair, indent),
                    // Rule::COMMENT => println!("{:?} -{}", subpair.as_rule(), subpair.as_str()),
                    Rule::EOI => (),
                    _ => println!("UNKNOWN {:?}", subpair.as_rule())
                }
            }
        }
        Err(error) => println!("error parsing: {}", error),
    }
}

fn parse_section(_section: Pair<Rule>, _indent: usize) {
    let indent = _indent + 1;
    for subpair in _section.into_inner() {
        println!("{}{:?}", SEPARATOR.repeat(indent), subpair.as_rule());
        match subpair.as_rule() {
            Rule::env_stmt => {
                parse_environment(subpair, indent);
            }
            Rule::cmd_stmt => {
                parse_cmd_stmt(subpair, indent);
            }
            Rule::expression => {
                parse_expression(subpair, indent);
            }
            Rule::section => parse_section(subpair, indent),
            Rule::COMMENT => println!("{}{}", SEPARATOR.repeat(indent), subpair.as_str()),
            _ => println!("UNKNOWN RULE: {:?}", subpair.as_rule()),
        }
    }
}

fn parse_environment(_env: Pair<Rule>, _indent: usize) {
    let indent = _indent + 1;
    for subpair in _env.into_inner() {
        match subpair.as_rule() {
            Rule::name => println!("{}{}", SEPARATOR.repeat(indent), subpair.as_str()),
            Rule::env_content => {
                println!("{}{:?}", SEPARATOR.repeat(indent), subpair.as_rule());
                for subsubpair in subpair.into_inner() {
                    match subsubpair.as_rule() {
                        Rule::section => parse_section(subsubpair, indent),
                        _ => unreachable!()
                    }
                }
            }
            _ => println!("{}UNEXPECTED RULE {:?}", SEPARATOR.repeat(indent), subpair.as_rule()),
        }
    }
}

fn parse_cmd_stmt(_stmt: Pair<Rule>, _indent:usize) {
    let indent = _indent + 1;
    for subpair in _stmt.into_inner() {
        match subpair.as_rule() {
            Rule::ctrl_character => (),
            Rule::name => println!("{}{}", SEPARATOR.repeat(indent), subpair.as_str()),
            Rule::cmd_stmt_opt => println!("{}{}",SEPARATOR.repeat(indent),  subpair.as_str()),
            Rule::expression => parse_expression(subpair, indent),
            _ => println!("{} unexpected: {:?}", SEPARATOR.repeat(indent), subpair.as_rule()),
        }
    }
}

fn parse_expression(_expr: Pair<Rule>, _indent:usize) {
    let indent = _indent + 1;
    for subpair in _expr.into_inner() {
        match subpair.as_rule() {
            Rule::cmd_stmt => parse_cmd_stmt(subpair, indent),
            Rule::literal => println!("{}literal: {}", SEPARATOR.repeat(indent), subpair.as_str()),
            _ => unreachable!()
        }
    }
}
