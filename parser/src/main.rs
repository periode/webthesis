pub mod foliage;
pub mod parsers;

use std::fs::{self};
use std::path::Path;
use std::time::Instant;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use clap::Parser as ArgParser;
use parsers::text::Node;

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
    println!("- reading: {}", args.input);


    let start = Instant::now();

    println!("- parsing: text...");
    let ast = parsers::text::parse(src.clone());

    println!("- parsing: table of contents...");
    let toc = parsers::toc::parse(src.clone());

    println!("- parsing: listings...");
    let mut state = parsers::listings::State::new();
    let listings = parsers::listings::parse(src.clone(), &mut state);

    let duration = start.elapsed();

    if args.verbosity == 1 {
        pretty_print(&ast, 0);
    }

    parsers::text::save(ast, &args.output_dir);
    parsers::toc::save(toc, &args.output_dir);
    parsers::listings::save(listings, &args.output_dir);
    println!("- lasting: {:?}", duration)
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

#[test]
fn it_parses_a_file() {
    let test_src = fs::read_to_string("test_inputs/basic.tex").expect("Cannot open file");
    let test_ast = parsers::text::parse(test_src);
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
