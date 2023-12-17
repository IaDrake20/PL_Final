/*
Syntactical Analysis
Implement the main parser using recursive descent. Use a Pratt parser for expression
parsing. Optionally, add error localization and descriptions.
 */

use crate::machine::{self, Machine};
use crate::token::{Token, self};
use crate::lexer::Lexer;
use crate::parse_tree::ParseTree;
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, IfNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
use std::{env, string};
use std::rc::Rc;
use std::fs;
use std::fs::File;
use std::io::Read;
//use std::fs::File;
//use std::io::{self, Read};

const INDENT : usize = 2;



pub struct DescentParser {
    lexer: Lexer,
    indent: usize,
    tree: ParseTree,
}
pub struct PrattParser {
    lexer: Lexer,
}

impl Token {

    fn binding_powers(token : &Token) -> (i32, i32) {
        match token {

            Token::PAREN_L => (2,1),
            Token::PAREN_R => (1,1),

            Token::COLON => (2,3),

            Token::OP_ADD => (2,3),
            Token::OP_SUB => (2,3),
            Token::OP_MUL => (4,5),
            Token::OP_DIV => (4,5),

            Token::OP_EQ => (2,1),
            Token::OP_LT => (2,1),
            Token::OP_GT => (2,1),
            Token::OP_NEQ => (2,1),
            Token::OP_NLT => (2,1),
            Token::OP_NGT => (2,1),

            Token::OP_NOT => (5,6),
            Token::OP_AND => (4,5),
            Token::OP_OR => (2,3),

            Token::OP_ASSIGN=> (2,1),

            Token::ID(_) => (1,1),

            Token::LIT_I32(_) => (1,1),
            Token::LIT_F32(_) => (1,1),
            Token::LIT_CHAR(_) => (1,1),
            Token::LIT_STRING(_) => (1,1),
            Token::LIT_BOOL(_) => (1,1),

            Token::EOI => (0,0),
            _ => {
                panic!("Missing binding powers for token {:?}", token);
            }
        }
    }

    fn left_bp(&self) -> i32 { Token::binding_powers(self).0 }
    fn right_bp(&self) -> i32 { Token::binding_powers(self).1 }

}

impl PrattParser {
    pub fn new(lexer : Lexer) -> PrattParser {
        PrattParser { lexer }
    }

    pub fn analyze(&mut self) -> ParseTree {
        self.pratt_driver(Token::EOI.right_bp() )
    }

    fn pratt_driver(&mut self, requested_bp: i32) -> ParseTree {
        let mut current_token = self.current();
        self.advance();

        let mut left_denotation = self.func_prefix(current_token.clone());

        if (Token::ID(String::new()) == current_token.clone() ) & ( self.current() == Token:: PAREN_L){
            left_denotation = ParseTree::new(current_token.clone());
            let temp = self.current();
            left_denotation.push(self.func_call(temp.clone()));
            self.advance();
        }

        if left_denotation.token == Token::PAREN_L {
            left_denotation.push(ParseTree::new(Token::PAREN_R));
            self.advance();
        }

        loop {
            current_token = self.current();
            // compare binding powers
            if requested_bp >= current_token.left_bp() {
                // finish subexpression (requested rbp >= curr lbp)
                return left_denotation;
            }
            // go on with subexpression (requested rbp < curr lbp)
            self.advance();
            left_denotation = self.func_infix(current_token, left_denotation);
        }
    }

    fn func_call(&mut self, token: Token) -> ParseTree {
        let mut output = ParseTree::new(self.current());
        self.advance();
        if self.peek(Token::PAREN_R) {
            return output;
        }
        if self.peek(Token::id()){
            let mut token_string : String = "".to_string();
            while (self.current() != Token::COMMA) & (self.current() != Token::PAREN_R){
                token_string.push_str(&(self.current().string().to_string() + " "));
                self.advance();
            }
            let token_str : &'static str = token_string.leak();

            let prattlexer = Lexer::new(token_str);
            let mut prattparser = PrattParser::new(prattlexer);
            output.push(prattparser.analyze());
        }
        else if self.peek(Token::lit_i32()){
            let mut token_string : String = "".to_string();
            while (self.current() != Token::COMMA) & (self.current() != Token::PAREN_R){
                token_string.push_str(&(self.current().string().to_string() + " "));
                self.advance();
            }
            let token_str : &'static str = token_string.leak();

            let prattlexer = Lexer::new(token_str);
            let mut prattparser = PrattParser::new(prattlexer);
            output.push(prattparser.analyze());
        }
        else if self.peek(Token::lit_f32()){
            let mut token_string : String = "".to_string();
            while (self.current() != Token::COMMA) & (self.current() != Token::PAREN_R){
                token_string.push_str(&(self.current().string().to_string() + " "));
                self.advance();
            }
            let token_str : &'static str = token_string.leak();

            let prattlexer = Lexer::new(token_str);
            let mut prattparser = PrattParser::new(prattlexer);
            output.push(prattparser.analyze());
        }
        while self.accept(Token::COMMA) {
            if self.peek(Token::id()){
                let mut token_string : String = "".to_string();
                while (self.current() != Token::COMMA) & (self.current() != Token::PAREN_R){
                    token_string.push_str(&(self.current().string().to_string() + " "));
                    self.advance();
                }
                let token_str : &'static str = token_string.leak();
    
                let prattlexer = Lexer::new(token_str);
                let mut prattparser = PrattParser::new(prattlexer);
                output.push(prattparser.analyze());
            }
            else if self.peek(Token::lit_i32()){
                let mut token_string : String = "".to_string();
                while (self.current() != Token::COMMA) & (self.current() != Token::PAREN_R){
                    token_string.push_str(&(self.current().string().to_string() + " "));
                    self.advance();
                }
                let token_str : &'static str = token_string.leak();
    
                let prattlexer = Lexer::new(token_str);
                let mut prattparser = PrattParser::new(prattlexer);
                output.push(prattparser.analyze());
            }
            else if self.peek(Token::lit_f32()){
                let mut token_string : String = "".to_string();
                while (self.current() != Token::COMMA) & (self.current() != Token::PAREN_R){
                    token_string.push_str(&(self.current().string().to_string() + " "));
                    self.advance();
                }
                let token_str : &'static str = token_string.leak();
    
                let prattlexer = Lexer::new(token_str);
                let mut prattparser = PrattParser::new(prattlexer);
                output.push(prattparser.analyze());
            }
        }
        
        return output;
        
    }

    fn func_prefix(&mut self, token: Token) -> ParseTree {
        match token {
            Token::ID(_) => {
                ParseTree::new(token.clone())
            }
            Token::LIT_I32(_) => {
                ParseTree::new(token.clone())
            }
            Token::LIT_F32(_) => {
                ParseTree::new(token.clone())
            }
            Token::LIT_CHAR(_) => {
                ParseTree::new(token.clone())
            }
            Token::LIT_STRING(_) => {
                ParseTree::new(token.clone())
            }
            Token::LIT_BOOL(_) => {
                ParseTree::new(token.clone())
            }
            Token::TYPE_I32 => {
                ParseTree::new(token.clone())
            }
            Token::TYPE_CHAR => {
                ParseTree::new(token.clone())
            }
            Token::TYPE_F32 => {
                ParseTree::new(token.clone())
            }
            Token::PAREN_R => {
                ParseTree::new(token.clone())
            }
            Token::PAREN_L => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(right_denotation);
                return node;
            }
            Token::OP_NOT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(right_denotation);
                return node;
            }
            Token::KW_FUNC => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(right_denotation);
                return node;
            }
            Token::OP_SUB => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(right_denotation);
                return node;
            }
            //Token::OP_ADD => { todo!() }
            //Token::OP_ASSIGN => { todo!() }
            Token::EOI => {
                ParseTree::new(token.clone())
            }
            _ => {
                panic!("Missing prefix function for token {:?}", token);
            }
        }
    }

    fn func_infix(&mut self, token: Token, left_denotation : ParseTree) -> ParseTree {
        match token {
            //Token::LIT_I32(_) => { todo!() }
            Token::OP_ADD => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::COLON => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_AND => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_OR => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_EQ => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_LT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_GT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_NEQ => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_NLT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_NGT => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_SUB => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_DIV => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_MUL => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::OP_ASSIGN => {
                let mut node = ParseTree::new(token.clone());
                let right_denotation = self.pratt_driver(token.right_bp());
                node.push(left_denotation);
                node.push(right_denotation);
                return node;
            }
            Token::EOI => { todo!() }
            _ => {
                panic!("Missing infix function for token {:?}", token);
            }
        }
    }
}

impl PrattParser { // utility functions for lexer

    fn current(&mut self) -> Token {
        self.lexer.current()
    }

    fn advance(&mut self) {
        self.lexer.advance();
    }

    fn expect(&mut self, symbol: Token) -> ParseTree{
        let output: ParseTree;
        if self.current() == symbol {
            output = ParseTree::new(self.current());
            self.advance();
        } else {

            println!("{:?}", self.current());
            println!("{:?}", symbol);

            panic!("Did not expect {:?}!", self.current());
        }

        return output;
    }

    // similar to expect, but doesnt crash if expected symbol is not found
    // returns boolean
    fn accept(&mut self, symbol: Token) -> bool {
        if self.current() == symbol {
            self.advance();
            true
        } else {
            false
        }
    }

    // checks that current symbol is equivalent to a given symbol
    // used for block lists
    fn peek(&mut self, symbol: Token) -> bool {
        self.lexer.current() == symbol
    }
}
impl DescentParser {  // simple recursive descend parser
pub fn new(lexer: Lexer) -> DescentParser {
    DescentParser {
        lexer,
        indent: 0,
        tree: ParseTree::new(Token::EOI),
    }
}

    pub fn analyze(&mut self) {

        println!("GOT THIS FAR!!");

        let mut program = ProgramNode::new();
        self.indent = 0;
        while self.peek(Token::KW_FUNC) {
            self.tree = ParseTree::new(Token::KW_FUNC);
            self.tree = self.parse_func(self.tree.clone());
            
            self.tree.print();
            // make call to transform function node into executables
            
            program.func_nodes.push(Rc::new(ParseTree::funcNode_grow(&self.tree)));
        }
        if self.peek(Token::EOI) {
            self.expect(Token::EOI);
            self.tree = ParseTree::new(Token::EOI);


            self.tree.print();
            let rcProgram = Rc::new(program);
            let machine = Machine::new(rcProgram);
    
            machine.run();

            return;
        }
        
    }

    // parse_func  -> KW_FUNC ID() <parse_parameter_list> ARROW_R ID() <parse_block_nest>
    fn parse_func(&mut self, mut tree: ParseTree) -> ParseTree {
        {
            self.advance();
            tree.push(self.parse_parameter_list());
            if self.peek(Token::ARROW_R) {
                tree.push(self.expect(Token::ARROW_R));
                tree.push(self.expect(Token::TYPE_I32));
            }
            tree.push(self.parse_block_nest());
        }
        return tree;
    }

    // parse_parameter_list -> PAREN_L PAREN_R | PAREN_L parse_parameter [(COMMA <parse_parameter>) repeats until no COMMA found] PAREN_R
    fn parse_parameter_list(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        self.advance();
        output.push(self.expect(Token::PAREN_L));
        if self.peek(Token::PAREN_R) {
            output.push(self.expect(Token::PAREN_R));
            return output;
        }
        output.push(self.parse_parameter());
        while self.accept(Token::COMMA) {
            output.push(self.parse_parameter());
        }
        output.push(self.expect(Token::PAREN_R));
        return output;
    }

    // parse_parameter -> ID() COLON <TYPE_I32 | TYPE_CHAR | TYPE_F32>  [ Complete, there are no nonTerminals remaining ]
    fn parse_parameter(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        output.push(self.expect(Token::id()));

        if (self.peek(Token::COLON)){
            output.push(self.expect(Token::COLON));
            if self.peek(Token::TYPE_I32) {
                output.push(self.expect(Token::TYPE_I32));
            }
            if self.peek(Token::TYPE_F32) {
                output.push(self.expect(Token::TYPE_F32));
            }
            if self.peek(Token::TYPE_CHAR) {
                output.push(self.expect(Token::TYPE_CHAR));
            }
        }
        return output;
    }

    // parse_block_nest -> BRACKET_L BRACKET_R | BRACKET_L <parse_block_list, parse_if, parse_while, parse_else, parse_statement, parse_print, parse_expression> BRACKET_R
    fn parse_block_nest(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        self.advance();
        while ! self.peek(Token::BRACKET_R) {
            if self.peek(Token::BRACKET_L) {
                output.push(self.parse_block_list());
            }
            if self.peek(Token::IF) {
                output.push(self.parse_if());
                while self.peek(Token::ELSE) {
                    output.push(self.parse_else());
                }
            }
            if self.peek(Token::WHILE) {
                output.push(self.parse_while());
            }
            if self.peek(Token::LET) | self.peek(Token::RETURN){
                output.push(self.parse_statement());
                self.expect(Token::SEMICOLON);
            }
            if self.peek(Token::PRINT) {
                output.push(self.parse_print());
                self.expect(Token::SEMICOLON);
            }
            if self.peek(Token::id()) {
                output.push(self.parse_expression());//IAN:Removed false from params
                self.expect(Token::SEMICOLON);
            }
        }
        output.push(self.expect(Token::BRACKET_R));

        return output;
    }

    // parse_block_list -> <parse_block_nest> | <parse_block_nest> <parse_block_list>
    fn parse_block_list(&mut self) -> ParseTree{
        let mut output = self.parse_block_nest();
        if self.peek(Token::BRACKET_L) {
            output.push(self.parse_block_list());
        }
        return output;
    }

    // parse_expression -> (uses pratt parser to read expression)
    fn parse_expression(&mut self) -> ParseTree{
        let mut token_string : String = "".to_string();
        
        while (self.curr() != Token::SEMICOLON) & (self.curr() != Token::BRACKET_L){
            token_string.push_str(&(self.curr().string().to_string() + " "));
            self.advance();
        }
        let token_str : &'static str = token_string.leak();

        let prattlexer = Lexer::new(token_str);
        let mut prattparser = PrattParser::new(prattlexer);
        let mut output =  prattparser.analyze();

        return output;

    }

    // parse_if -> IF <parse_expression> BRACKET_L <parse_statement, parse_if, parse_print, parse_while, parse_expression> BRACKET_R
    fn parse_if(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        self.advance();
        output.push(self.parse_expression());
        output.push(self.expect(Token::BRACKET_L));
        while ! self.peek(Token::BRACKET_R) {
            if self.peek(Token::LET) | self.peek(Token::RETURN){
                output.push(self.parse_statement());
                self.expect(Token::SEMICOLON);
            }
            else if self.peek(Token::IF) {
                output.push(self.parse_if());
                while self.peek(Token::ELSE) {
                    output.push(self.parse_else());
                }
            }
            else if self.peek(Token::PRINT) {
                output.push(self.parse_print());//
                self.expect(Token::SEMICOLON);
            }
            else if self.peek(Token::WHILE) {
                output.push(self.parse_while());
            }
            else if self.peek(Token::id()) {
                output.push(self.parse_expression());
                self.expect(Token::SEMICOLON);
            }
        }
        output.push(self.expect(Token::BRACKET_R));
        if(self.peek(Token::ELSE)){
            output.push(self.parse_else());
        }

        return output;
    }


    // parse_while -> WHILE LIT_BOOL() BRACKETL <parse_expression> BRACKET_R
    fn parse_while(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        self.advance();
        output.push(self.expect(Token::BRACKET_L));
        while ! self.peek(Token::BRACKET_R) {
            if self.peek(Token::LET) | self.peek(Token::RETURN){
                output.push(self.parse_statement());
                self.expect(Token::SEMICOLON);
            }
            else if self.peek(Token::WHILE) {
                output.push(self.parse_while());
            }
            else if self.peek(Token::IF) {
                output.push(self.parse_if());
                while self.peek(Token::ELSE) {
                    output.push(self.parse_else());
                }
            }
            else if self.peek(Token::PRINT) {
                output.push(self.parse_print());
                self.expect(Token::SEMICOLON);
            }
            else if self.peek(Token::id()) {
                output.push(self.parse_expression());
                self.expect(Token::SEMICOLON);
            }
        }
        output.push(self.expect(Token::BRACKET_R));

        return output;
    }

    // parse_else -> ELSE <parse_if> | ELSE BRACKET_L <parse_let | parse_return | parse_if> BRACKET_R
    fn parse_else(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        self.advance();

        if self.peek(Token::IF) {
            output.push(self.parse_if()); //
        }
        else {
            output.push(self.expect(Token::BRACKET_L));
            while ! self.peek(Token::BRACKET_R) {
                if self.peek(Token::LET) | self.peek(Token::RETURN){
                    output.push(self.parse_statement());
                    self.expect(Token::SEMICOLON);
                }
                else if self.peek(Token::WHILE) {
                    output.push(self.parse_while());
                }
                else if self.peek(Token::IF) {
                    output.push(self.parse_if());
                    while self.peek(Token::ELSE) {
                        output.push(self.parse_else());
                    }
                }
                else if self.peek(Token::PRINT) {
                    output.push(self.parse_print());
                    self.expect(Token::SEMICOLON);
                }
                else if self.peek(Token::id()) {
                    output.push(self.parse_expression());
                    self.expect(Token::SEMICOLON);
                }
            }
            output.push(self.expect(Token::BRACKET_R));
        }
        return output;
    }

    // parse_statement -> RETURN <parse_expression> SEMICOLON
    fn parse_statement(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        self.advance();
        output.push(self.parse_expression());
        return output;
    }

    // parse_print -> PRINT parse_expression SEMICOLON
    fn parse_print(&mut self) -> ParseTree{
        let mut output = ParseTree::new(self.curr());
        self.advance();
        output.push(self.parse_expression());
        return output;
    }
}

impl DescentParser { // utility functions for lexer
    // these functions just call to the lexer versions of the functions

    fn curr(&mut self) -> Token {
        self.lexer.current()
    }

    fn advance(&mut self) {
        self.lexer.advance();
    }

    // called to check that next symbol is the correct symbol,
    // advances if so
    // remember that the self.curr is not the actual current token, but is instead the one coming next
    // when initialized, current token is first token
    // tokens with a stored value inside are considered equivalent to the same token type, regardless of inner value
    fn expect(&mut self, symbol: Token) -> ParseTree{
        let output: ParseTree;
        if self.curr() == symbol {
            output = ParseTree::new(self.curr());
            self.advance();
        } else {

            println!("{:?}", self.curr());
            println!("{:?}", symbol);

            panic!("Did not expect {:?}!", self.curr());
        }

        return output;
    }

    // similar to expect, but doesnt crash if expected symbol is not found
    // returns boolean
    fn accept(&mut self, symbol: Token) -> bool {
        if self.curr() == symbol {
            self.advance();
            true
        } else {
            false
        }
    }

    // checks that current symbol is equivalent to a given symbol
    // used for block lists
    fn peek(&mut self, symbol: Token) -> bool {
        self.lexer.current() == symbol
    }
}


impl DescentParser { 
// prints self at correctly indented location
fn indent_print(&mut self, msg: &'static str) {
    println!("{:<indent$}{:}", "", msg, indent=self.indent);
}

    // self explanatory
    fn indent_increment(&mut self) {
        self.indent += INDENT;
    }
    fn indent_decrement(&mut self) {
        self.indent -= INDENT;
    }

}
pub fn main() {

    let file_path = "";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // create a sequence of tokens that is assumed to
    //   be output of the lexer
    let tokens = contents;

    // create Pratt parser
    let lexer = Lexer::new(&tokens);
    let mut parser = PrattParser::new(lexer);

    // start Pratt top-down operator precedence parsing
    let tree = parser.analyze();

    // print parse tree
    tree.print();

    // create a sequence of tokens that is assumed to
    //   be output of the lexer

    // create input for lexer

    /*
    let input = r#"
    func add(x : int32, y : char, z : f32) -> int32
    [
        let value : int32 = 35;
        if true [
            let a = 5;
    ]
        else [
            let b = 7;
    ]
        return value;
    ]

    func repeat() -> i32 [
        while [
            if (a and b) [
                print("both are true");
            ]
            else if (a) [
                print("b is false");
            ]
            else [
                print("a is false");
                x = x + 5;
            ]
        ]

        return 3;
    ]

    func another(y : char) [
        print(y);
    ]

    func main()
    [
        let sum : int32 = 42;
        return sum;
    ]
    "#;
     */

    // create recursive descent parser
    let lexer = Lexer::new(&tokens);
    let mut parser = DescentParser::new(lexer);

    parser.analyze();
}