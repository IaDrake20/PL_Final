#![allow(non_snake_case)]

use std::{env, fs};
use std::rc::Rc;
use crate::machine::Machine;
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
use crate::value::Value;
use clap::Parser;
use std::path::PathBuf;

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

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

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
fn grow_ast_program0() -> Rc<ProgramNode> {
    let mut program = ProgramNode::new();

    // global variables
    let let_count = LetNode::new( "count".to_string(), Value::Nil);
    let let_help =  LetNode::new( "help".to_string(), Value::Nil);
    program.let_nodes.push(Rc::new(let_count));
    program.let_nodes.push(Rc::new(let_help));

    // add function
    let mut parameters_add = vec![];
    parameters_add.push(Parameter::new("a".to_string()));
    parameters_add.push(Parameter::new("b".to_string()));

    let mut block_add = BlockNode::new();
    let stmtAdd1 = StmtNode::Return(
        ReturnNode::new(ExprNode::Add(
            Rc::new(ExprNode::Var("a".to_string())),
            Rc::new(ExprNode::Var("b".to_string())),
        ))
    );
    block_add.statements.push(Rc::new(stmtAdd1));

    let func_add = FuncNode::new(
        "add".to_string(),
        parameters_add,
        block_add);

    program.func_nodes.push(Rc::new(func_add));

    // main function
    let mut parameters_main = vec![];
    parameters_main.push(Parameter::new("argc".to_string()));

    let mut block_main = BlockNode::new();
    let stmtMain1 = StmtNode::Let(LetNode::new("sum".to_string(), Value::Nil));
    let stmtMain2 = StmtNode::Assign(
        AssignNode::new("sum".to_string(), ExprNode::Add(
           Rc::new(ExprNode::Val(Value::I32(3))),
           Rc::new(ExprNode::Add(
               Rc::new(ExprNode::Val(Value::I32(5))),
               Rc::new(ExprNode::Val(Value::I32(7))),
           ))
        ))
    );
    let stmtMain3 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    let stmtMain4 = StmtNode::Assign(AssignNode::new("sum".to_string(),
        ExprNode::Call("add".to_string(), vec![
            Rc::new(ExprNode::Var("sum".to_string())),
            Rc::new(ExprNode::Val(Value::I32(1)))
        ])
    ));
    let stmtMain5 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    block_main.statements.push(Rc::new(stmtMain1));
    block_main.statements.push(Rc::new(stmtMain2));
    block_main.statements.push(Rc::new(stmtMain3));
    block_main.statements.push(Rc::new(stmtMain4));
    block_main.statements.push(Rc::new(stmtMain5));

    let func_main = FuncNode::new(
        "main".to_string(),
        parameters_main,
        block_main);

    program.func_nodes.push(Rc::new(func_main));


    Rc::new(program)
}


/*

// The test AST corresponds to program1:

let count;

func factorial_recursion(n) [
    print n;
    if n < 2 [
        return 1;
    ] else [
        return n * factorial_recursion(n-1);
    ]
]

func main(argc) [
    let result;
    result = factorial_recursion(5);
    print result;
]


 */

//=======
use crate::token::Token;
use crate::lexer::Lexer;
use crate::parse_tree::ParseTree;
use crate::parser::DescentParser;
const INDENT : usize = 2;

fn main() {

    let mut file_path: PathBuf = Default::default();
    let mut path = String::from("%USERPROFILE\\Documents\\Github\\PL_Final\\text.txt");

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

    file_path.push(&path); // Append the modified path to file_path

    println!("Reading file from path: {:?}", file_path);

    let input = fs::read_to_string(&file_path)
        .expect("Could not read the file");

    //println!("File contents:\n{}", input);

    {
        //let args = Cli::parse();
        //let content = std::fs::read_to_string(&args.path).expect("could not read file");

        // create a sequence of tokens that is assumed to
        //   be output of the lexer

        // create input for lexer
    }

    let input = r#"
    
    func main() [
        print factorial_recursion(6);
    ]
    
    func factorial_recursion(n)
    [
        if n < 2 [
            return 1;
        ] else [
            return n * factorial_recursion(n-1);
        ]
    ]
    "#;
    
    // create recursive descent parser
    let lexer = Lexer::new(&input);
    let mut parser = DescentParser::new(lexer);

    parser.analyze();

    
}
