#![allow(dead_code)]

use std::alloc::GlobalAlloc;
use std::rc::{Rc, self};
use clap::builder::NonEmptyStringValueParser;
//use rand::distributions::Exp;

use crate::token::{Token, self};
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, WhileNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
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

    pub fn funcNode_grow(&self) -> FuncNode{
        let mut parameters = vec![];
        for n in 1..self.children[0].children.len() - 1{
            parameters.push(Parameter::new(self.children[0].children[n].token.string().parse().unwrap()));
        }

        let mut block = BlockNode::new();
        let mut stmt: StmtNode;

        for n in 0..self.children[1].children.len() - 1 {
            stmt = self.children[1].children[n].stmtNode_grow();
            block.statements.push(Rc::new(stmt));
        }

        let func = FuncNode::new(
            self.children[0].token.string().to_string(),
            parameters,
            block);
        
        return func;
    }

    pub fn stmtNode_grow(&self) -> StmtNode {
        match self.token {
            Token::RETURN => {
                return StmtNode::Return(ReturnNode::new(self.children[0].exprNode_grow()));
            },
            Token::IF => {
                let mut trueBlock = BlockNode::new();
                let mut falseBlock = BlockNode::new();
                let mut stmt: StmtNode;

                for mut n in 2..self.children.len(){
                    if self.children[n].token == Token::BRACKET_R {
                        break;
                    } 
                    else {
                        stmt = self.children[n].stmtNode_grow();
                        trueBlock.statements.push(Rc::new(stmt));
                    }
                }
                
                for mut n in 1..self.children[self.children.len() - 1].children.len() - 1 {
                    stmt = self.children[self.children.len() - 1].children[n].stmtNode_grow();
                    falseBlock.statements.push(Rc::new(stmt));
                }
                
                return StmtNode::If(IfNode::new(self.children[0].exprNode_grow(), trueBlock, falseBlock));
            },
            Token::WHILE => {
                let mut trueBlock = BlockNode::new();
                let mut falseBlock = BlockNode::new();
                let mut stmt: StmtNode;

                for mut n in 2..self.children.len(){
                    if self.children[n].token == Token::BRACKET_R {
                        break;
                    } 
                    else {
                        stmt = self.children[n].stmtNode_grow();
                        trueBlock.statements.push(Rc::new(stmt));
                    }
                }
                
                if(self.children.len() > 0){
                    if(self.children[self.children.len() - 1].children.len() > 0){
                        for mut n in 1..self.children[self.children.len() - 1].children.len() - 1 {
                            stmt = self.children[self.children.len() - 1].children[n].stmtNode_grow();
                            falseBlock.statements.push(Rc::new(stmt));
                        }
                    }
                }
                
                return StmtNode::While(WhileNode::new(self.children[0].exprNode_grow(), trueBlock));},
            Token::PRINT => {
                return StmtNode::Print(PrintNode::new(self.children[0].exprNode_grow()));
            },
            Token::OP_ASSIGN => {
                return StmtNode::Assign(AssignNode::new(self.children[0].token.string().parse::<String>().unwrap(), self.children[1].exprNode_grow()));
            },
            Token::LET => {
                return StmtNode::Let(LetNode::new(self.children[0].token.string().to_string(),Value::Nil));
            },
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
                if(self.children.len() > 0){
                    let mut exprs = vec![];
                    for n in 0..self.children[0].children.len(){
                        exprs.insert(n, Rc::new(self.children[0].children[n].exprNode_grow()));
                    }
                    return ExprNode::Call(self.token.string().to_string(), exprs)
                }
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
