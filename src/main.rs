#![allow(non_snake_case)]

use std::{env, fs};
use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::ops::Deref;
use std::rc::Rc;
use crate::machine::Machine;
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
use crate::value::Value;
use clap::{arg, Parser};
use std::path::PathBuf;
use clap::builder::PossibleValue;
use log::LevelFilter;
use log::{debug, error, log_enabled, info, Level};
use clap_logger::{ClapInitLogger, ClapLoglevelArg};
use clap::{Arg, Command};

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file: PathBuf,

    #[clap(short = 'p', long = "parse", group = "action")]
    parse: bool,

    #[clap(short = 'e', long = "execute", group = "action")]
    execute: bool,

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
    let matches = Command::new("Tiny Language Interpreter")
        .version("1.0")
        .author("IBAC")
        .about("KYS, FYS -Brady")
        .next_line_help(true)
        .arg(arg!(--one <VALUE>).required(false))
        .arg(arg!(--two <VALUE>).required(false))
        .get_matches();

    let args = Cli::parse();
    if args.parse {
        println!("[ALERT] WIP");
        println!("Parsing file: {:?}", args.file);
    }

    if args.execute {
        println!("[ALERT] WIP");
        println!("executing: {:?}", args.file);
    }

    if args.run {
        println!("Running file: {:?}", args.file);
        //let mut file_path: PathBuf = Default::default();
        //let mut path = String::from("%USERPROFILE\\Documents\\Github\\PL_Final\\text.txt");

        /*
        if let Some(home_dir) = env::var_os("HOME") {
            file_path = PathBuf::from(home_dir);
            path = String::from("text.txt"); // Modify the filename
        } else if let Some(userprofile) = env::var_os("USERPROFILE") {
            file_path = PathBuf::from(userprofile);
            path = String::from("Documents\\Github\\PL_Final\\text.txt"); // Modify the path and filename for Windows
        } else {
            println!("Home directory not found");
            return;
        }

         */

        let mut file_path = args.file.deref();
        println!("Reading file from path: {:?}", file_path);

        let input = fs::read_to_string(&file_path)
            .expect("Could not read the file");

        // create recursive descent parser
        let lexer = Lexer::new(&input);
        let mut parser = DescentParser::new(lexer);

        parser.analyze();
    }

}
