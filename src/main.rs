#![allow(non_snake_case)]

use std::{env, fs};
use std::error::Error;
use std::fs::read_to_string;
use std::ops::Deref;
use clap::{arg, Parser};
use std::path::PathBuf;
use clap_logger::{ClapInitLogger, ClapLoglevelArg};

mod tree;
mod parse_tree;
mod executor;
mod machine;
mod analyzer;
mod symbols;
mod frame;
mod value;
mod evaluator;
mod parser;
mod token;
mod lexer;
use crate::token::Token;
use crate::lexer::Lexer;
use crate::parse_tree::ParseTree;
use crate::parser::DescentParser;
const INDENT : usize = 2;

/// Program CLI
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input File
    file: PathBuf,

    /// Show parser output
    #[clap(short = 'p', long = "parse", group = "action")]
    parse: bool,

    /// Show execute output
    #[clap(short = 'e', long = "execute", group = "action")]
    execute: bool,

    /// Show run output
    #[clap(short = 'r', long = "run", group = "action")]
    run: bool,
}

enum Logger {
    info(String),
    debug,
    warn,
    none

}

fn main() {
    let args = Cli::parse();
    let cpy = args.file.clone();

    let input = read_to_string(args.file).expect("Failed to read input file.");
    println!("{:?}", input);

    if args.parse {
        println!("[ALERT] WIP");
        //println!("Parsing file: {:?}", args.file);
    }

    if args.execute {
        println!("[ALERT] WIP");
        //println!("executing: {:?}", args.file);
    }

    if args.run {

        let mut file_path = PathBuf::new();
        file_path.push(cpy.deref());
        //println!("Reading file from path: {:?}", file_path);

        let input = fs::read_to_string(&file_path)
            .expect("Could not read the file");

        // create recursive descent parser
        let lexer = Lexer::new(&input);
        let mut parser = DescentParser::new(lexer);

        parser.analyze();

        println!("HIQFREFRNFBBWBEBVWTVBBTBVWTRBWRTBREWBBEBEBYEBTB");
    }
}