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

    println!("parsing...");
    match LaTeXParser::parse(Rule::document, &src) {
        Ok(mut pairs) => {
            let result = pairs.next().unwrap();

            for section in result.into_inner() {
                println!("\n{:?}", section.as_rule());
                match section.as_rule() {
                    Rule::section => parse_section(section),
                    _ => println!("nothing"),
                }
            }
        }
        Err(error) => println!("error parsing: {}", error),
    }

    // println!("parsed {:?}\n\n{}", result.as_rule(), result.as_str());
}

fn parse_section(_section: Pair<Rule>) {
    for field in _section.into_inner() {
        println!("  {:?}", field.as_rule());
        match field.as_rule() {
            Rule::environment_stmt => {
                parse_environment(field);
            }
            Rule::command_stmt => {
                for subfield in field.into_inner() {
                    println!("      {:?} - {}", subfield.as_rule(), subfield.as_str())
                }
            }
            Rule::expression => {
                parse_expression(field);
            }
            Rule::section => parse_section(field),
            Rule::comment => println!("     {}", field.as_str()),
            _ => println!("UNKNOWN RULE: {:?}", field.as_rule()),
        }
    }
}

fn parse_environment(_env: Pair<Rule>) {
    for elem in _env.into_inner() {
        match elem.as_rule() {
            Rule::environment_begin => println!("      {:?}", elem.as_rule()),
            Rule::environment_content => {
                println!("      {:?}", elem.as_rule());
                parse_section(elem);
            }
            Rule::environment_end => println!("      {:?}", elem.as_rule()),
            _ => println!("      unexpected environment bourdel"),
        }
    }
}

fn parse_expression(_expr: Pair<Rule>) {
    for subfield in _expr.into_inner() {
        println!("      {:?} - {}", subfield.as_rule(), subfield.as_str())
    }
}
