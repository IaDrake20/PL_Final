use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::evaluator::Evaluator;
use crate::frame::Frame;
use crate::tree::{BlockNode, FuncNode, ProgramNode, StmtNode};
use crate::value::Value;

enum Control {
    Next,
    Return,
    Break,
    Continue
}

pub struct Executor {
    program: Rc<ProgramNode>,
}

impl Executor {

    pub fn new(program: Rc<ProgramNode>) -> Executor {
        Executor { program }
    }

    pub fn execute(&self) {
        println!("[info] Execute.");
        self.execute_program();
    }

    fn execute_program(&self) {
        println!("[info] Execute Program.");

        // get program node symbol table
        let rc_symbols = self.program.symbols.clone();
        let symbols = rc_symbols.borrow();

        // find main function node
        let rc_main = if let Some(main) = symbols.map.get("main") {
            match &main.value {
                Value::Func(rc_main, _) => { rc_main.clone() }
                _ => { panic!("Symbol 'main' is not a function!"); }
            }
        } else {
            panic!("Cannot find 'main' symbol!");
        };

        // create global stack frame
        let mut global = Frame::new(None);
        global.init_symbols(symbols.deref());
        let rc_global = Rc::new(RefCell::new(global));

        // execute main function
        let arguments = vec![Value::I32(1)];
        Self::execute_function(rc_main, rc_global, arguments);
    }

    pub fn execute_function(
        rc_func: Rc<FuncNode>,
        globals: Rc<RefCell<Frame>>,
        arguments: Vec<Value>
    ) -> Value
    {
        let name = &rc_func.name;
        println!("[debug] calling function '{name}'.");

        // create local stack frame
        let mut locals = Frame::new(Some(globals));

        // initialize parameters
        let name = &rc_func.name;
        if rc_func.numParameters() > arguments.len() {
            panic!("Not enough arguments for function {name}!");
        }
        if rc_func.numParameters() < arguments.len() {
            panic!("To many arguments for function {name}!");
        }
        locals.init_parameters(&rc_func.parameters, arguments);

        // execute function block
        let rc_block = rc_func.block_node.clone();
        let rc_locals = Rc::new(RefCell::new(locals));
        let (_, value) = Self::execute_block_without_scope(rc_block, rc_locals);

        value
    }

    fn execute_block_without_scope(
        rc_block: Rc<BlockNode>,
        rc_locals: Rc<RefCell<Frame>>,
    ) -> (Control, Value) {

        // execute statements
        for statement in &rc_block.statements {
            let (control, value) = Self::execute_statement(
                statement.clone(),
                rc_locals.clone(),
            );
            match control {
                Control::Next => {}
                Control::Return => { return (Control::Return, value) }
                Control::Break => {}
                Control::Continue => {}
            }
        }

        (Control::Next, Value::Nil)
    }

    fn execute_statement(
        rc_statement: Rc<StmtNode>,
        rc_locals: Rc<RefCell<Frame>>,
    ) -> (Control, Value)
    {
        match rc_statement.deref() {
            StmtNode::Let(_) => {
                println!("[debug] ignoring let statement");
                (Control::Next, Value::Nil)
            }
            StmtNode::Assign(assign) => {
                println!("[debug] executing assign statement");
                let name = &assign.name;
                let value = Evaluator::evaluate(assign.expr.clone(), rc_locals.clone());
                rc_locals.borrow_mut().assign(name, value);
                (Control::Next, Value::Nil)
            }
            StmtNode::If(ifNode) => {
                println!("[debug] executing if statement");
                let value_cond = Evaluator::evaluate(
                    ifNode.cond.clone(), rc_locals.clone());
                if let Value::Bool(b) = value_cond {
                    if b {
                        Self::execute_block_without_scope(
                            ifNode.block_node_true.clone(), rc_locals.clone())
                    } else {
                        Self::execute_block_without_scope(
                            ifNode.block_node_false.clone(), rc_locals.clone())
                    }
                } else {
                    panic!("If-then-else statement condition must be of type boolean!");
                }
            }
            StmtNode::Return(ret) => {
                println!("[debug] executing return statement");
                let value = Evaluator::evaluate(ret.expr.clone(), rc_locals.clone());
                (Control::Return, value)
            }
            StmtNode::Print(print) => {
                println!("[debug] executing print statement");
                let value = Evaluator::evaluate(print.expr.clone(), rc_locals.clone());
                value.print();
                (Control::Next, Value::Nil)
            }

        }

    }

}