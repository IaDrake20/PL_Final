#![allow(dead_code)]

use std::rc::Rc;
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

    pub fn grow(&self, mut program: ProgramNode){
        match self.token {
            Token::KW_FUNC => {
                // add function
                let mut parameters = vec![];
                for n in 1..self.children[0].children.len() {
                    parameters.push(Parameter::new(self.children[0].children[n].token.string().parse().unwrap()));
                }

                let mut block = BlockNode::new();
                let mut stmt = ();//= self.children[1].children[1].grow(program.clone());

                for n in 1..self.children[1].children.len() {
                    stmt = self.children[1].children[n].grow(program.clone());
                }

                block.statements.push(Rc::new(stmt));

                let func = FuncNode::new(
                    self.children[0].token.string().to_string(),
                    parameters,
                    block);

                program.func_nodes.push(Rc::new(func));
            }
            Token::PRINT => {todo!()}
            Token::RETURN => {todo!()}
            Token::IF => {todo!()}
            Token::ELSE => {todo!()}
            Token::WHILE => {todo!()}
            _ => {

            }
        }
    }
}
