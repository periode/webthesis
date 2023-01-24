// from https://github.com/bign86/pest_latex

use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "latex-grammar.pest"]
pub struct LaTeXParser;


fn main() {
    let input = "latex_test.tex";
    let src = fs::read_to_string(input).expect("Cannot open file");

    println!("parsing");
    let result = LaTeXParser::parse(Rule::document, &src)
                            .expect("Error").next().unwrap();

    // println!("parsed {:?}\n\n{}", result.as_rule(), result.as_str());

    for section in result.into_inner() {
        println!("\n{:?}", section.as_rule());
        match section.as_rule() {
            Rule::section => {
                for field in section.into_inner() {
                    println!("  {:?} - {}",field.as_rule(), field.as_str());
                    for subfield in field.into_inner() {
                        println!("      {:?} - {}", subfield.as_rule(), subfield.as_str())
                    }
                }
            }
            Rule::control_statement => {
                println!("direct control statement")
            }
            _ => println!("nothing")
        }
    }
}



