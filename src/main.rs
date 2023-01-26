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
                    Rule::env_stmt => parse_environment(subpair),
                    // Rule::COMMENT => println!("{:?} -{}", subpair.as_rule(), subpair.as_str()),
                    Rule::EOI => (),
                    _ => println!("UNKNOWN {:?}", subpair.as_rule())
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
            Rule::env_stmt => {
                parse_environment(subpair);
            }
            Rule::cmd_stmt => {
                parse_cmd_stmt(subpair);
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
            Rule::env_begin => println!("      {:?}", subpair.as_rule()),
            Rule::env_content => {
                println!("      {:?}", subpair.as_rule());
                for subsubpair in subpair.into_inner() {
                    match subsubpair.as_rule() {
                        Rule::section => parse_section(subsubpair),
                        _ => unreachable!()
                    }
                }
            }
            Rule::env_end => println!("      {:?}", subpair.as_rule()),
            _ => println!("      unexpected environment bourdel"),
        }
    }
}

fn parse_cmd_stmt(_stmt: Pair<Rule>) {
    for subpair in _stmt.into_inner() {
        match subpair.as_rule() {
            Rule::ctrl_character => (),
            Rule::name => println!("    {}", subpair.as_str()),
            Rule::cmd_stmt_opt => println!("    {}", subpair.as_str()),
            Rule::expression => parse_expression(subpair),
            _ => println!("      unexpected: {:?}", subpair.as_rule()),
        }
    }
}

fn parse_expression(_expr: Pair<Rule>) {
    for subpair in _expr.into_inner() {
        match subpair.as_rule() {
            Rule::cmd_stmt => parse_cmd_stmt(subpair),
            Rule::literal => println!("     literal: {}", subpair.as_str()),
            _ => unreachable!()
        }
    }
}
