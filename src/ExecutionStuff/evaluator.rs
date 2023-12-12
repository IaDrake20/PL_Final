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
            ExprNode::LessThan(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::LessThan)
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
            Value::Bool(a) => { panic!("Left operand of '{op:?}' is Bool!"); }
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
                    Value::F32(_) => { todo!() }
                    Value::Chars(_) => { todo!() }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { panic!("Left operand of '{op:?}' is Func!"); }
        }
    }

    fn relational(value_a: Value, value_b: Value, op : RelationalOp) -> Value {
        match value_a {
            Value::Nil => { panic!("Left operand of '{op:?}' is Nil!"); }
            Value::Bool(a) => { panic!("Left operand of '{op:?}' is Bool!"); }
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
                        }
                    }
                    Value::F32(_) => { todo!() }
                    Value::Chars(_) => { todo!() }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { panic!("Left operand of '{op:?}' is Func!"); }
        }
    }

}