#![allow(dead_code)]

use std::rc::Rc;
use clap::builder::NonEmptyStringValueParser;
use rand::distributions::Exp;

use crate::token::{Token, self};
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};


#[derive(Clone)]
pub struct ParseTree {
    pub token : Token,
    pub children : Vec<Box<ParseTree>>
}

impl ParseTree {

    pub fn new(token : Token) -> ParseTree {
        ParseTree {
            token,
            children : vec![]
        }
    }

    pub fn push(&mut self, tree : ParseTree) {
        self.children.push(Box::<ParseTree>::new(tree));
    }

    pub fn node_string(&self) -> String {
        format!("{:?}", self.token)
    }

    fn print_recursively(&self, level : usize) {
        let shift = 2*level;
        print!("{:1$}", "", shift);
        println!("{}", self.node_string());
        for child in &self.children {
            child.as_ref().print_recursively(level+1);
        }
    }

    pub fn print(&self) {
        self.print_recursively(0);
    }

    pub fn funcNode_grow(&self, mut program: ProgramNode){
        let mut parameters = vec![];
        for n in 1..self.children[0].children.len() {
            parameters.push(Parameter::new(self.children[0].children[n].token.string().parse().unwrap()));
        }

        let mut block = BlockNode::new();
        let mut stmt: StmtNode;

        for n in 1..self.children[1].children.len() {
            stmt = self.children[1].children[n].stmtNode_grow();
            block.statements.push(Rc::new(stmt));
        }

        let func = FuncNode::new(
            self.children[0].token.string().to_string(),
            parameters,
            block);

        program.func_nodes.push(Rc::new(func));
    }

    pub fn stmtNode_grow(&self) -> StmtNode {
        match self.token {
            Token::RETURN => { todo!()},
            Token::IF => { todo!()},
            Token::ELSE => {todo!()},
            Token::WHILE => {todo!()},
            Token::PRINT => {todo!()},
            Token::OP_ASSIGN => {todo!()},
            Token::LET => {todo!()},
            _ => {panic!()}
        }
    }

    pub fn exprNode_grow(&self) -> ExprNode {
        match self.token {
            Token::PAREN_L => {todo!()},
            Token::PAREN_R => {todo!()},
            Token::BRACKET_L => {todo!()},
            Token::BRACKET_R => {todo!()},
            Token::BRACE_L => {todo!()},
            Token::BRACE_R => {todo!()},

            Token::COLON => {todo!()},

            Token::OP_ADD => {todo!()},
            Token::OP_SUB => {todo!()},
            Token::OP_MUL => {todo!()},
            Token::OP_DIV => {todo!()},

            Token::OP_EQ => {todo!()},
            Token::OP_LT => {todo!()},
            Token::OP_GT => {todo!()},
            Token::OP_NEQ => {todo!()},
            Token::OP_NLT => {todo!()},
            Token::OP_NGT => {todo!()},

            Token::OP_NOT => {todo!()},
            Token::OP_AND => {todo!()},
            Token::OP_OR => {todo!()},

            Token::OP_ASSIGN=> {todo!()},

            Token::ID(_) => {todo!()},

            Token::LIT_I32(_) => {todo!()},
            Token::LIT_F32(_) => {todo!()},
            Token::LIT_CHAR(_) => {todo!()},
            Token::LIT_STRING(_) => {todo!()},
            Token::LIT_BOOL(_) => {todo!()},
            _ => {panic!()}
        }
    }
}
