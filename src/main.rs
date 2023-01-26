// from https://github.com/bign86/pest_latex

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;

fn main() {
    let input = "latex_test.tex";
    let src = fs::read_to_string(input).expect("Cannot open file");

    println!("parsing {}...", input);
    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();

            for subpair in pair.into_inner() {
                println!("\n{:?}", subpair.as_rule());
                match subpair.as_rule() {
                    Rule::section => parse_section(subpair),
                    Rule::EOI => (),
                    _ => unreachable!()
                }
            }
        }
        Err(error) => println!("error parsing: {}", error),
    }
}

fn parse_section(_section: Pair<Rule>) {
    for subpair in _section.into_inner() {
        println!("  {:?}", subpair.as_rule());
        match subpair.as_rule() {
            Rule::environment_stmt => {
                parse_environment(subpair);
            }
            Rule::command_stmt => {
                parse_command_stmt(subpair);
            }
            Rule::expression => {
                parse_expression(subpair);
            }
            Rule::section => parse_section(subpair),
            Rule::COMMENT => println!("     {}", subpair.as_str()),
            _ => println!("UNKNOWN RULE: {:?}", subpair.as_rule()),
        }
    }
}

fn parse_environment(_env: Pair<Rule>) {
    for subpair in _env.into_inner() {
        match subpair.as_rule() {
            Rule::environment_begin => println!("      {:?}", subpair.as_rule()),
            Rule::environment_content => {
                println!("      {:?}", subpair.as_rule());
                parse_section(subpair);
            }
            Rule::environment_end => println!("      {:?}", subpair.as_rule()),
            _ => println!("      unexpected environment bourdel"),
        }
    }
}

fn parse_command_stmt(_stmt: Pair<Rule>) {
    for subpair in _stmt.into_inner() {
        println!("      {:?} - {}", subpair.as_rule(), subpair.as_str())
    }
}

fn parse_expression(_expr: Pair<Rule>) {
    for subpair in _expr.into_inner() {
        match subpair.as_rule() {
            Rule::command_stmt => parse_command_stmt(subpair),
            Rule::literal => println!("     literal: {}", subpair.as_str()),
            _ => unreachable!()
        }
    }
}
