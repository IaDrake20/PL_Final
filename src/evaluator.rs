use core::panic;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::executor::Executor;
use crate::frame::Frame;
use crate::tree::ExprNode;
use crate::value::Value;

#[derive(Debug, Clone)]
enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Clone)]
enum RelationalOp {
    Equal,
    LessThan,
    GreaterThan,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,
    Not,
    And,
    Or
}



pub struct Evaluator {

}

impl Evaluator {

    pub fn evaluate(expr: Rc<ExprNode>, rc_frame: Rc<RefCell<Frame>>) -> Value {
        match expr.deref() {
            ExprNode::Var(name) => {
                rc_frame.borrow().lookup(name)
            }
            ExprNode::Val(value) => {
                value.clone()
            }
            ExprNode::Add(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::arithmetic(value_a, value_b, ArithmeticOp::Add)
            }
            ExprNode::Sub(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::arithmetic(value_a, value_b, ArithmeticOp::Sub)
            }
            ExprNode::Mul(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::arithmetic(value_a, value_b, ArithmeticOp::Mul)
            }
            ExprNode::Div(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::arithmetic(value_a, value_b, ArithmeticOp::Div)
            }
            ExprNode::Equal(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::Equal)
            }
            ExprNode::LessThan(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::LessThan)
            }
            ExprNode::GreaterThan(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::GreaterThan)
            }
            ExprNode::NotEqual(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::NotEqual)
            }
            ExprNode::LessThanEqual(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::LessThanEqual)
            }
            ExprNode::GreaterThanEqual(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::GreaterThanEqual)
            }
            ExprNode::Not(expr_a) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b: Value = value_a.clone();
                Self::relational(value_a, value_b, RelationalOp::And)
            }
            ExprNode::And(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::And)
            }
            ExprNode::Or(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::Or)
            }
            ExprNode::Call(name, rc_exprs) => {
                println!("[debug] evaluating call '{name}'");
                match rc_frame.borrow().lookup_global(name) {
                    Value::Func(rc_func, argc) => {
                        
                        
                        assert_eq!(argc,rc_exprs.len());

                        let mut arguments = vec![];
                        for rc_expr in rc_exprs {
                            let arg = Self::evaluate(rc_expr.clone(), rc_frame.clone());
                            arguments.push(arg);
                        }

                        if let Some(globals) = rc_frame.borrow().get_globals() {
                            Executor::execute_function(rc_func, globals, arguments)
                        } else {
                            panic!("Can't find globals in current frame!");
                        }
                    }
                    _ => {
                        panic!("Can't find function '{name}' in globals!");
                    }
                }
            }
        }
    }

    fn arithmetic(value_a: Value, value_b: Value, op : ArithmeticOp) -> Value {
        match value_a {
            Value::Nil => { panic!("Left operand of '{op:?}' is Nil!"); }
            Value::Bool(a) => {panic!(); }
            Value::I32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::I32(a + b) }
                            ArithmeticOp::Sub => { Value::I32(a - b) }
                            ArithmeticOp::Mul => { Value::I32(a * b) }
                            ArithmeticOp::Div => { Value::I32(a / b) }
                        }
                    }
                    Value::F32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::F32((a as f32) + b) }
                            ArithmeticOp::Sub => { Value::F32((a as f32) - b) }
                            ArithmeticOp::Mul => { Value::F32((a as f32) * b) }
                            ArithmeticOp::Div => { Value::F32((a as f32) / b) }
                        }
                    }
                    Value::Chars(b) => {
                        if b.len() <= 1 {
                            match op {
                                //IAN: replaced ints with i32 and introduced parseing it from b
                                ArithmeticOp::Add => { Value::I32(a + (b.parse::<i32>().unwrap())) }
                                ArithmeticOp::Sub => { Value::I32(a - (b.parse::<i32>().unwrap())) }
                                ArithmeticOp::Mul => { Value::I32(a * (b.parse::<i32>().unwrap())) }
                                ArithmeticOp::Div => { Value::I32(a / (b.parse::<i32>().unwrap())) }
                            }
                        }
                        else {
                            //IAN: fixed some type issues here with int from string shenanigans
                            match op {
                                ArithmeticOp::Add => { Value::Chars((a + b.parse::<i32>().unwrap()).to_string()) }
                                ArithmeticOp::Sub => { Value::Chars((a - b.parse::<i32>().unwrap()).to_string()) }
                                ArithmeticOp::Mul => { Value::Chars((a * b.parse::<i32>().unwrap()).to_string()) }
                                ArithmeticOp::Div => { Value::Chars((a / b.parse::<i32>().unwrap()).to_string()) }
                            }
                        }
                    }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::F32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::F32(a + (b as f32)) }
                            ArithmeticOp::Sub => { Value::F32(a - (b as f32)) }
                            ArithmeticOp::Mul => { Value::F32(a * (b as f32)) }
                            ArithmeticOp::Div => { Value::F32(a / (b as f32)) }
                        }
                    }
                    Value::F32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::F32(a + b) }
                            ArithmeticOp::Sub => { Value::F32(a - b) }
                            ArithmeticOp::Mul => { Value::F32(a * b) }
                            ArithmeticOp::Div => { Value::F32(a / b) }
                        }
                    }
                    Value::Chars(b) => {
                        if b.len() <= 1 {
                            match op {
                                //IAN: changed int to u8 and added parsing it from b
                                ArithmeticOp::Add => { Value::F32(a + (b.parse::<f32>().unwrap())) }
                                ArithmeticOp::Sub => { Value::F32(a - (b.parse::<f32>().unwrap())) }
                                ArithmeticOp::Mul => { Value::F32(a * (b.parse::<f32>().unwrap())) }
                                ArithmeticOp::Div => { Value::F32(a / (b.parse::<f32>().unwrap())) }
                            }
                        }
                        else {
                            match op {
                                ArithmeticOp::Add => { Value::Chars((a + (b.parse::<f32>().unwrap())).to_string()) }
                                ArithmeticOp::Sub => { Value::Chars((a - (b.parse::<f32>().unwrap())).to_string()) }
                                ArithmeticOp::Mul => { Value::Chars((a * (b.parse::<f32>().unwrap())).to_string()) }
                                ArithmeticOp::Div => { Value::Chars((a / (b.parse::<f32>().unwrap())).to_string()) }
                            }
                        }
                    }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::Chars(a) => {
                if a.len() <= 1{
                    match value_b {
                        Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                        Value::Bool(b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                        Value::I32(b) => {
                            match op {
                                //IAN: changed all ints to u8s and introduced parse calls frm a
                                ArithmeticOp::Add => { Value::Chars(String::from(((a.parse::<i32>().unwrap()) + b).to_string())) }
                                ArithmeticOp::Sub => { Value::Chars(String::from(((a.parse::<i32>().unwrap()) - b).to_string())) }
                                ArithmeticOp::Mul => { Value::Chars(String::from(((a.parse::<i32>().unwrap()) * b).to_string())) }
                                ArithmeticOp::Div => { Value::Chars(String::from(((a.parse::<i32>().unwrap()) / b).to_string())) }
                            }
                        }
                        Value::F32(b) => {
                            match op {
                                //IAN: changed int to u8 and added parsing it from a, rust suggested building thr chars as String::from
                                ArithmeticOp::Add => { Value::Chars(String::from(((a.parse::<f32>().unwrap()) + b).to_string())) }
                                ArithmeticOp::Sub => { Value::Chars(String::from(((a.parse::<f32>().unwrap()) - b).to_string())) }
                                ArithmeticOp::Mul => { Value::Chars(String::from(((a.parse::<f32>().unwrap()) * b).to_string())) }
                                ArithmeticOp::Div => { Value::Chars(String::from(((a.parse::<f32>().unwrap()) / b).to_string())) }
                            }
                        }
                        Value::Chars(b) => {
                            match op {
                                //IAN: changed int to u8 and added parsing it from a, rust suggested building thr chars as String::from
                                ArithmeticOp::Add => { Value::Chars(String::from(((a.parse::<u8>().unwrap()) + (b.parse::<u8>().unwrap())) as char)) }
                                ArithmeticOp::Sub => { Value::Chars(String::from(((a.parse::<u8>().unwrap()) - (b.parse::<u8>().unwrap())) as char)) }
                                ArithmeticOp::Mul => { Value::Chars(String::from(((a.parse::<u8>().unwrap()) * (b.parse::<u8>().unwrap())) as char)) }
                                ArithmeticOp::Div => { Value::Chars(String::from(((a.parse::<u8>().unwrap()) / (b.parse::<u8>().unwrap())) as char)) }
                            }
                        }
                        Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                    }
                }
                else {
                    match value_b {
                        Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                        Value::Bool(b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                        Value::I32(b) => {
                            match op {
                                ArithmeticOp::Add => { Value::Chars((a.parse::<i32>().unwrap() + b).to_string()) }
                                ArithmeticOp::Sub => { Value::Chars((a.parse::<i32>().unwrap() - b).to_string()) }
                                ArithmeticOp::Mul => { Value::Chars((a.parse::<i32>().unwrap() * b).to_string()) }
                                ArithmeticOp::Div => { Value::Chars((a.parse::<i32>().unwrap() / b).to_string()) }
                            }
                        }
                        Value::F32(b) => {
                            match op {
                                //IAN: I'm not sure how well parsing into a float will go, guess we'll see
                                ArithmeticOp::Add => { Value::Chars((a.parse::<f32>().unwrap() + b).to_string()) }
                                ArithmeticOp::Sub => { Value::Chars((a.parse::<f32>().unwrap() - b).to_string()) }
                                ArithmeticOp::Mul => { Value::Chars((a.parse::<f32>().unwrap() * b).to_string()) }
                                ArithmeticOp::Div => { Value::Chars((a.parse::<f32>().unwrap() / b).to_string()) }
                            }
                        }
                        Value::Chars(b) => {
                            match op {
                                //IAN: these should be some kind of literals right?
                                ArithmeticOp::Add => { Value::Chars(a + &*b) }
                                _ => {panic!("Cannot perform '{op:?}' on strings"); }
                            }
                        }
                        Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                    }

                }
                
            }
            Value::Func(_, _) => { panic!("Left operand of '{op:?}' is Func!"); }
        }
    }

    fn relational(value_a: Value, value_b: Value, op : RelationalOp) -> Value {
        match value_a {
            Value::Nil => { panic!("Left operand of '{op:?}' is Nil!"); }
            Value::Bool(a) => {
                match value_b {
                    Value::Bool(b) => {
                        match op {
                            RelationalOp::And => { Value::Bool(a & b) }
                            RelationalOp::Or => { Value::Bool(a | b) }
                            RelationalOp::Not => { Value::Bool(! a) }
                            _ => { panic!(); }
                        }
                    }
                    _ => { panic!(); }
                }
            }
            Value::I32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            RelationalOp::Equal => { Value::Bool(a == b) }
                            RelationalOp::LessThan => { Value::Bool(a < b) }
                            RelationalOp::GreaterThan => { Value::Bool(a > b) }
                            RelationalOp::NotEqual => { Value::Bool(a != b) }
                            RelationalOp::LessThanEqual => { Value::Bool(a <= b) }
                            RelationalOp::GreaterThanEqual => { Value::Bool(a >= b) }
                            _ => { panic!(); }
                        }
                    }
                    Value::F32(b) => {
                        match op {
                            //IAN: Added safecasting because Rust was complaining
                            RelationalOp::Equal => { Value::Bool((a as f32) == b) }
                            RelationalOp::LessThan => { Value::Bool(a < b as i32) }
                            RelationalOp::GreaterThan => { Value::Bool(a > b as i32) }
                            RelationalOp::NotEqual => { Value::Bool((a as f32) != b) }
                            RelationalOp::LessThanEqual => { Value::Bool(a <= b as i32) }
                            RelationalOp::GreaterThanEqual => { Value::Bool(a >= b as i32) }
                            _ => { panic!(); }
                        }
                    }
                    Value::Chars(_) => { todo!() }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::F32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            //IAN: safe casted the ints to floats
                            RelationalOp::Equal => { Value::Bool(a == (b as f32)) }
                            RelationalOp::LessThan => { Value::Bool(a < b as f32) }
                            RelationalOp::GreaterThan => { Value::Bool(a > b as f32) }
                            RelationalOp::NotEqual => { Value::Bool(a != (b as f32)) }
                            RelationalOp::LessThanEqual => { Value::Bool(a <= b as f32) }
                            RelationalOp::GreaterThanEqual => { Value::Bool(a >= b as f32) }
                            _ => { panic!(); }
                        }
                    }
                    Value::F32(b) => {
                        match op {
                            RelationalOp::Equal => { Value::Bool(a == b) }
                            RelationalOp::LessThan => { Value::Bool(a < b) }
                            RelationalOp::GreaterThan => { Value::Bool(a > b) }
                            RelationalOp::NotEqual => { Value::Bool(a != b) }
                            RelationalOp::LessThanEqual => { Value::Bool(a <= b) }
                            RelationalOp::GreaterThanEqual => { Value::Bool(a >= b) }
                            _ => { panic!(); }
                        }
                    }
                    Value::Chars(_) => { todo!() }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { panic!("Left operand of '{op:?}' is Func!"); }
        }
    }

}