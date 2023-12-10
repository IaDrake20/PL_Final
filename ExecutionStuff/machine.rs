use std::rc::Rc;
use crate::analyzer::Analyzer;
use crate::executor::Executor;
use crate::tree::ProgramNode;

pub struct Machine {
    rc_program: Rc<ProgramNode>,
}

impl Machine {

    pub fn new(rc_program: Rc<ProgramNode>) -> Machine {
        Machine {
            rc_program
        }
    }

    pub fn run(&self) {

        let analyzer = Analyzer::new(self.rc_program.clone());
        analyzer.analyze(); // go to analyzer.rs line 17  -> i think analyzer is selfexplanatory

        let executor = Executor::new(self.rc_program.clone());
        executor.execute(); // go to executor.rs line 19  -> i think i get executor? it looks like we dont have to do much to it. 

    }
}