#![allow(dead_code)]

use std::rc::Rc;
use clap::builder::NonEmptyStringValueParser;
use rand::distributions::Exp;

use crate::token::{Token, self};
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
use crate::value::Value;


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
            Token::RETURN => {
                return StmtNode::Return(ReturnNode::new(self.children[0].exprNode_grow()));
            },
            Token::IF => { todo!()},
            Token::ELSE => {todo!()},
            Token::WHILE => {todo!()},
            Token::PRINT => {
                return StmtNode::Print(PrintNode::new(self.children[0].exprNode_grow()));
            },
            Token::OP_ASSIGN => {
                return StmtNode::Assign(AssignNode::new(self.children[0].token.string() as str, self.children[1].exprNode_grow()));
            },
            Token::LET => {
                return StmtNode::Let(LetNode::new(self.children[0].token.string() as str, self.children[1].exprNode_grow()));
            },
            _ => {panic!()}
        }
    }

    pub fn exprNode_grow(&self) -> ExprNode {
        match self.token {
            Token::PAREN_L => {
                return self.children[0].exprNode_grow();
            },
            Token::PAREN_R => 
            {},
            Token::OP_ADD => {
                return ExprNode::Add(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());
            },
            Token::OP_SUB => {
                if self.children.len() > 1 {
                    return ExprNode::Sub(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());
                }
                else {
                    return ExprNode::Sub(0, self.children[0].exprNode_grow());
                }
            }
            Token::OP_MUL => {
                return ExprNode::Mul(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());
            },
            Token::OP_DIV => {
                return ExprNode::Div(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());
            },

            Token::OP_EQ => {
                return ExprNode::Equal(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},
            Token::OP_LT => {
                return ExprNode::LessThan(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},
            Token::OP_GT => {
                return ExprNode::GreaterThan(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},
            Token::OP_NEQ => {
                return ExprNode::NotEqual(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},
            Token::OP_NLT => {
                return ExprNode::GreaterThanEqual(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},
            Token::OP_NGT => {
                return ExprNode::LessThanEqual(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},

            Token::OP_NOT => {
                return ExprNode::Not(self.children[0].exprNode_grow());},
            Token::OP_AND => {
                return ExprNode::And(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},
            Token::OP_OR => {
                return ExprNode::Or(self.children[0].exprNode_grow(), self.children[1].exprNode_grow());},

            Token::ID(_) => {
                return ExprNode::Var(self.token.string().to_string());
            },

            Token::LIT_I32(_) => {
                return ExprNode::Val(Value::I32(self.token.string() as i32));
            },
            Token::LIT_F32(_) => {
                return ExprNode::Val(Value::F32(self.token.string() as f32));
            },
            Token::LIT_CHAR(_) => {
                return ExprNode::Val(Value::Chars(self.token.string() as char));
            },
            Token::LIT_STRING(_) => {
                return ExprNode::Val(Value::Chars(self.token.string() as str));
            },
            Token::LIT_BOOL(_) => {
                return ExprNode::Val(Value::Bool(self.token.string() as bool));
            },
            _ => {panic!()}
        }
    }
}
