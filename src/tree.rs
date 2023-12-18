use std::cell::RefCell;
use std::rc::Rc;
use crate::symbols::Symbols;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub symbols: Rc<RefCell<Symbols>>,
    pub let_nodes: Vec<Rc<LetNode>>,
    pub func_nodes: Vec<Rc<FuncNode>>,
}

impl ProgramNode {
    pub fn new() -> ProgramNode {
        ProgramNode {
            symbols: Rc::new(RefCell::new(Symbols::new(None))),
            let_nodes: vec![],
            func_nodes: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncNode {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub block_node: Rc<BlockNode>,
}

impl FuncNode {

    pub fn new(name: String, parameters: Vec<Parameter>, block_node: BlockNode) -> FuncNode {
        FuncNode {
            name,
            parameters,
            block_node : Rc::new(block_node),
        }
    }

    pub fn numParameters(&self) -> usize {
        self.parameters.len()
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
}

impl Parameter {
    pub fn new(name: String) -> Parameter {
        Parameter {
            name
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockNode {
    pub symbols: Rc<RefCell<Symbols>>,
    pub statements: Vec<Rc<StmtNode>>,
}

impl BlockNode {
    pub fn new() -> BlockNode {
        BlockNode {
            symbols: Rc::new(RefCell::new(Symbols::new(None))),
            statements: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum StmtNode {
    Let(LetNode),
    Assign(AssignNode),
    If(IfNode),
    Return(ReturnNode),
    Print(PrintNode),
    While(WhileNode),
}


#[derive(Debug, Clone)]
pub struct LetNode {
    pub name: String,
    pub value: Value,
}

impl LetNode {
    pub fn new(name: String, value: Value) -> LetNode {
        LetNode {
            name,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssignNode {
    pub name: String,
    pub expr: Rc<ExprNode>,
}

impl AssignNode {
    pub fn new(name: String, expr: ExprNode) -> AssignNode {
        AssignNode {
            name,
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfNode {
    pub cond: Rc<ExprNode>,
    pub block_node_true: Rc<BlockNode>,
    pub block_node_false: Rc<BlockNode>,
}

impl IfNode {
    pub fn new(cond: ExprNode, block_node_true: BlockNode, block_node_false: BlockNode) -> IfNode {
        IfNode {
            cond: Rc::new(cond),
            block_node_true: Rc::new( block_node_true),
            block_node_false: Rc::new( block_node_false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub cond: Rc<ExprNode>,
    pub block_node_true: Rc<BlockNode>,
}

impl WhileNode {
    pub fn new(cond: ExprNode, block_node_true: BlockNode) -> WhileNode {
        WhileNode {
            cond: Rc::new(cond),
            block_node_true: Rc::new( block_node_true),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintNode {
    pub expr: Rc<ExprNode>,
}

impl PrintNode {
    pub fn new(expr: ExprNode) -> PrintNode {
        PrintNode {
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub expr: Rc<ExprNode>,
}

impl ReturnNode {
    pub fn new(expr: ExprNode) -> ReturnNode {
        ReturnNode {
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprNode {
    Var(String),
    Val(Value),
    Add(Rc<ExprNode>, Rc<ExprNode>),
    Sub(Rc<ExprNode>, Rc<ExprNode>),
    Mul(Rc<ExprNode>, Rc<ExprNode>),
    Div(Rc<ExprNode>, Rc<ExprNode>),
    Equal(Rc<ExprNode>, Rc<ExprNode>),
    LessThan(Rc<ExprNode>, Rc<ExprNode>),
    GreaterThan(Rc<ExprNode>, Rc<ExprNode>),
    NotEqual(Rc<ExprNode>, Rc<ExprNode>),
    LessThanEqual(Rc<ExprNode>, Rc<ExprNode>),
    GreaterThanEqual(Rc<ExprNode>, Rc<ExprNode>),
    Not(Rc<ExprNode>),
    And(Rc<ExprNode>, Rc<ExprNode>),
    Or(Rc<ExprNode>, Rc<ExprNode>),
    Call(String, Vec<Rc<ExprNode>>),
}

impl ExprNode {


}


