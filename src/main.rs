#![allow(non_snake_case)]

use std::{env, fs};
use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
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

/*
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the file to read
    file: PathBuf,
    /// Logging level
    #[arg(short, long, default_value = "info", value_parser = vec![PossibleValue::new("info"), PossibleValue::new("debug"), PossibleValue::new("warn"), PossibleValue::new("none")], group = "action")]
    loglevel: String,
    tokenize: bool,
    parse: bool,
    execute: bool,
}
*/


/*

// The test AST corresponds to program0:

let count;
let help;

func add(a,b) [
    return a + b;
]

func main(argc) [
    let sum;
    sum = 3+(5+7);
    print sum;
    sum = add(sum, 1);
    print sum;
]


 */


//=======
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

}
