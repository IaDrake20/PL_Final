#![allow(non_snake_case)]

use std::rc::Rc;
use crate::machine::Machine;
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
use crate::value::Value;

mod tree;
mod executor;
mod machine;
mod analyzer;
mod symbols;
mod frame;
mod value;
mod evaluator;

// The primary files that actually do a thing are the main, machine, analyzer, and executor. everything else is supporting those. 

/*

The test AST corresponds to following code:

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

// 1) how does one translate a parse tree into the a program as written in the below function


fn grow_ast_program0() -> Rc<ProgramNode> { // fixed ast for above string -> most likely needs scrapped in place of parser, but may be useful in understanding how to implement
                                                // he mentions in the instructions that the parser need fsm, recursive descent, and pratt. correct me if im wrong, but my parser already has that right?
                                                    // like wouldnt we just import the existing parser and the lexical and syntactical analysis are just done?
    let mut program = ProgramNode::new();

    // global variables
    let let_count = LetNode::new( "count".to_string(), Value::Nil); // when a let node is added, manually push it to the vector in the program node, it does not appear to be placed on the program node
    let let_help =  LetNode::new( "help".to_string(), Value::Nil);
    program.let_nodes.push(Rc::new(let_count)); // the let_nodes vector is primarily used to make sure variables were declared before being used (Semantical Analysis)
    program.let_nodes.push(Rc::new(let_help));      // unsure if its already implemented

    // add function
    let mut parameters_add = vec![];
    parameters_add.push(Parameter::new("a".to_string()));
    parameters_add.push(Parameter::new("b".to_string()));

    let mut block_add = BlockNode::new();  // the blockNode is the interior of the function node
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

    program.func_nodes.push(Rc::new(func_add));  // when a func node is added, manually push it to the vector in the program node 
                                                    // the func_nodes vector will be used for calling functions from anywhere else in code. inherently allows recursion (i think)

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


fn run0() {
    let rc_program = grow_ast_program0();  

    let runtime = Machine::new(rc_program);
    runtime.run(); // go to machine.rs line 18
}


fn main() {
    run0();
}