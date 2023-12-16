#![allow(dead_code)]

use std::rc::{Rc, self};
use clap::builder::NonEmptyStringValueParser;
use rand::distributions::Exp;

use crate::token::{Token, self};
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
use crate::value::Value;
use crate::evaluator::Evaluator;


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
                return StmtNode::Assign(AssignNode::new(self.children[0].token.string().parse::<String>().unwrap(), self.children[1].exprNode_grow()));
            },
            Token::LET => {panic!()},

                //return StmtNode::Let(LetNode::new((self.children[0].token.string() as str).parse().unwrap(), self.children[1].exprNode_grow() as Value));
                
                // current block: how to get relevant frame for use in the evaluator call
/*
{
                return StmtNode::Let(
                    LetNode::new(
                        self.children[0].token.string().to_string(),
                        Evaluator::evaluate(Rc::new(self.children[1].exprNode_grow()), rc_locals.clone())
                    )
                );
            }
*/
            _ => {panic!()}
        }
    }

    pub fn exprNode_grow(&self) -> ExprNode {
        match self.token {
            Token::PAREN_L => {
                return self.children[0].exprNode_grow();
            },
            Token::PAREN_R => {panic!()},
            Token::OP_ADD => {
                return ExprNode::Add(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));
            },
            Token::OP_SUB => {
                if self.children.len() > 1 {
                    return ExprNode::Sub(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));
                }
                else {
                    return ExprNode::Sub(Rc::from(ExprNode::Val(Value::I32(0))), Rc::from(self.children[0].exprNode_grow()));
                }
            }
            Token::OP_MUL => {
                return ExprNode::Mul(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));
            },
            Token::OP_DIV => {
                return ExprNode::Div(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));
            },

            Token::OP_EQ => {
                return ExprNode::Equal(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},
            Token::OP_LT => {
                return ExprNode::LessThan(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},
            Token::OP_GT => {
                return ExprNode::GreaterThan(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},
            Token::OP_NEQ => {
                return ExprNode::NotEqual(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},
            Token::OP_NLT => {
                return ExprNode::GreaterThanEqual(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},
            Token::OP_NGT => {
                return ExprNode::LessThanEqual(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},

            Token::OP_NOT => {
                return ExprNode::Not(Rc::from(self.children[0].exprNode_grow()));},
            Token::OP_AND => {
                return ExprNode::And(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},
            Token::OP_OR => {
                return ExprNode::Or(Rc::from(self.children[0].exprNode_grow()), Rc::from(self.children[1].exprNode_grow()));},

            Token::ID(_) => {
                return ExprNode::Var(self.token.string().to_string());
            },

            Token::LIT_I32(_) => {
                return ExprNode::Val(Value::I32(self.token.string().parse::<i32>().unwrap()));
            },
            Token::LIT_F32(_) => {
                return ExprNode::Val(Value::F32(self.token.string().parse::<f32>().unwrap()));
            },
            Token::LIT_CHAR(_) => {
                return ExprNode::Val(Value::Chars(String::from(self.token.string().parse::<char>().unwrap())));
            },
            Token::LIT_STRING(_) => {
                return ExprNode::Val(Value::Chars((self.token.string() as &str).parse().unwrap()));
            },
            Token::LIT_BOOL(_) => {
                return ExprNode::Val(Value::Bool(self.token.string().parse::<bool>().unwrap()));
            },
            _ => {panic!()}
        }
    }
}
