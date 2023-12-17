#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Token {
    // Brackets:
    PAREN_L, PAREN_R, BRACKET_L, BRACKET_R, BRACE_L, BRACE_R,
    // Separators:
    POINT, COMMA, COLON, SEMICOLON, ARROW_R,
    // Arithmetic Ops:
    OP_ADD, OP_SUB, OP_MUL, OP_DIV,
    // Relational Ops:
    OP_EQ, OP_LT, OP_GT, OP_NEQ, OP_NLT, OP_NGT,
    // Logical Ops:
    OP_NOT, OP_AND, OP_OR,
    // Assignment:
    OP_ASSIGN,
    // Keywords:
    KW_FUNC, LET, IF, ELSE, WHILE, PRINT, RETURN,
    // Identifiers:
    ID(String),
    // Basic Types:
    TYPE_I32, TYPE_F32, TYPE_CHAR,
    // Literals:
    LIT_I32(i32), LIT_F32(f32), LIT_CHAR(char), LIT_STRING(String), LIT_BOOL(bool), // LIT_BOOL added for if-else statements
    // End-of-Input:
    EOI
}


impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for Token { }

impl Token {
    pub fn id() -> Token {
        Token::ID(String::new())
    }
    pub fn lit_i32() -> Token { Token::LIT_I32(0) }
    pub fn lit_f32() -> Token { Token::LIT_F32(0.0) }
    pub fn bool() -> Token {Token::LIT_BOOL(true)}


    pub fn string(&self) -> &str{
        match self {
            Token::PAREN_L => "(",
            Token::PAREN_R => ")",
            Token::BRACKET_L => "[",
            Token::BRACKET_R => "]",
            Token::BRACE_L => "{",
            Token::BRACE_R => "}",
            Token::POINT => ".",
            Token::COMMA => ",",
            Token::COLON => ":",
            Token::SEMICOLON =>";",
            Token::ARROW_R => "=>",
            Token::OP_ADD => "+",
            Token::OP_SUB => "-",
            Token::OP_MUL => "*",
            Token::OP_DIV => "/",
            Token::OP_EQ => "==",
            Token::OP_LT => "<",
            Token::OP_GT => ">",
            Token::OP_NEQ => "!=",
            Token::OP_NLT => ">=",
            Token::OP_NGT => "<=",
            Token::OP_NOT => "not",
            Token::OP_AND => "and",
            Token::OP_OR => "or",
            Token::OP_ASSIGN => "=",
            Token::KW_FUNC => "func",
            Token::LET => "let",
            Token::IF => "if",
            Token::ELSE => "else",
            Token::WHILE => "while",
            Token::PRINT => "print",
            Token::RETURN => "return",
            Token::TYPE_I32 => "i32",
            Token::TYPE_F32 => "f32",
            Token::TYPE_CHAR => "char",
            Token::EOI => "EOI",
            _ => {
                let mut output: String = format!("{:?}", self);
                if output.contains("LIT_"){
                    if output.contains("BOOL"){
                        output = output.strip_prefix("LIT_BOOL(").unwrap().to_string();
                        output = output.strip_suffix(")").unwrap().to_string();
                    }
                    else if output.contains("F32") {
                        output = output.strip_prefix("LIT_F32(").unwrap().to_string();
                        output = output.strip_suffix(")").unwrap().to_string();
                    }
                    else if output.contains("STRING") {
                        output = output.strip_prefix("LIT_STRING(").unwrap().to_string();
                        output = output.strip_suffix(")").unwrap().to_string();
                    }
                    else if output.contains("CHAR") {
                        output = output.strip_prefix("LIT_CHAR(").unwrap().to_string();
                        output = output.strip_suffix(")").unwrap().to_string();
                    }
                    else if output.contains("I32") {
                        output = output.strip_prefix("LIT_I32(").unwrap().to_string();
                        output = output.strip_suffix(")").unwrap().to_string();
                    }
                }
                else {
                    output = output.strip_prefix("ID(\"").unwrap().to_string();
                    output = output.strip_suffix("\")").unwrap().to_string();
                }

                return output.leak();
            }
        }
    }
}