use crate::token::Token;

pub struct Lexer {
    input_string: String,
    input_position: usize,
    current_state: i32,
    current_token: Token,
    buffer_string: String,
}


impl Lexer {

    // called from parser_descent.rs
    pub fn new(string_input: &str) -> Lexer {

        let mut lexicon = Lexer {
            input_string: string_input.to_string(),
            input_position: 0,
            current_state: 0,
            current_token: Token::SEMICOLON,
            buffer_string: "".to_string()
        };

        lexicon.advance();

        return lexicon;
    }

    // literally just a call
    pub fn current(&self) -> Token {
        self.current_token.clone()
    }

    // this version of advance moves through an array of tokens sets current token
    // my version goes through a string and sets current token to the found token

    pub fn advance(&mut self){

        self.current_state = 0;

        while self.current_state != -1 {

            if self.current_state == 0 {
                if self.input_position >= self.input_string.len() {
                    self.current_token = Token::EOI;
                    self.current_state = -1;
                }

                else {
                    self.current_state = 1;
                }
            }

            else if self.current_state == 1 {
                self.buffer_string.push(self.input_string.as_bytes()[self.input_position] as char);
                self.input_position = self.input_position + 1;

                match self.buffer_string.as_str(){
                    "(" => self.current_token = Token::PAREN_L,
                    ")" => self.current_token = Token::PAREN_R,
                    "[" => self.current_token = Token::BRACKET_L,
                    "]" => self.current_token = Token::BRACKET_R,
                    "{" => self.current_token = Token::BRACE_L,
                    "}" => self.current_token = Token::BRACE_R,
                    "." => self.current_token = Token::POINT,
                    "," => self.current_token = Token::COMMA,
                    ":" => self.current_token = Token::COLON,
                    ";" => self.current_token = Token::SEMICOLON,
                    "+" => self.current_token = Token::OP_ADD,
                    "*" => self.current_token = Token::OP_MUL,
                    "/" => self.current_token = Token::OP_DIV,
                    " " => self.current_state = 0,
                    "\n" => self.current_state = 0,
                    "\t" => self.current_state = 0,
                    _ => self.current_state = 2
                }

                if self.current_state == 0{
                    self.buffer_string = "".to_string()
                }

                if self.current_state == 1 {
                    self.current_state = -1;
                    self.buffer_string = "".to_string();
                }
            }

            else if self.current_state == 2 {

                if self.buffer_string.starts_with("<") {
                    if self.input_string.as_bytes()[self.input_position] as char == '=' {
                        self.current_token = Token::OP_NGT;
                        self.input_position = self.input_position + 1;
                    }
                    else {
                        self.current_token = Token::OP_LT;
                    }
                }
                else if self.buffer_string.starts_with(">") {
                    if self.input_string.as_bytes()[self.input_position] as char == '=' {
                        self.current_token = Token::OP_NLT;
                        self.input_position = self.input_position + 1;
                    }
                    else {
                        self.current_token = Token::OP_GT;
                    }
                }
                else if self.buffer_string.starts_with("!") {
                    if self.input_string.as_bytes()[self.input_position] as char == '=' {
                        self.current_token = Token::OP_NEQ;
                        self.input_position = self.input_position + 1;
                    }
                }
                else if self.buffer_string.starts_with("=") {
                    if self.input_string.as_bytes()[self.input_position] as char == '=' {
                        self.current_token = Token::OP_EQ;
                        self.input_position = self.input_position + 1;
                    }
                    else {
                        self.current_token = Token::OP_ASSIGN;
                    }
                }
                else if self.buffer_string.starts_with("-") {
                    if self.input_string.as_bytes()[self.input_position] as char == '>' {
                        self.current_token = Token::ARROW_R;
                        self.input_position = self.input_position + 1;
                    }
                    else {
                        self.current_token = Token::OP_SUB;
                    }
                }
                else {
                    self.current_state = 3;
                }

                if self.current_state == 2 {
                    self.current_state = -1;
                    self.buffer_string = "".to_string();
                }
            }

            else if self.current_state == 3 {
                let mut more: bool = true;

                if self.input_position == self.input_string.len() {
                    more = false;
                }

                if (self.input_string.as_bytes()[0] as char) == '\'' {
                    while more {
                        self.buffer_string.push(self.input_string.as_bytes()[self.input_position] as char);
                        self.input_position = self.input_position + 1;
                        if (self.input_position == self.input_string.len()) | ((self.buffer_string.as_bytes()[self.buffer_string.len() - 1] as char) == '\''){
                            more = false;
                        }
                    }
                    self.input_string = self.input_string.replace("\'", "");
                    self.current_token = Token::LIT_CHAR(self.input_string.parse::<char>().unwrap());
                }

                if (self.buffer_string.as_bytes()[0] as char) == '\"' {
                    while more {
                        self.buffer_string.push(self.input_string.as_bytes()[self.input_position] as char);
                        self.input_position = self.input_position + 1;
                        if (self.input_position == self.input_string.len()) | ((self.buffer_string.as_bytes()[self.buffer_string.len() - 1] as char) == '\"'){
                            more = false;
                        }
                    }
                    self.buffer_string = self.buffer_string.replace("\"", "");
                    self.current_token = Token::LIT_STRING(self.buffer_string.clone());
                }

                else {
                    while more {
                        if
                        ((self.input_string.as_bytes()[self.input_position] as char) >= 'A') & ((self.input_string.as_bytes()[self.input_position] as char) <= 'Z') |   // between A and Z
                            ((self.input_string.as_bytes()[self.input_position] as char) >= 'a') & ((self.input_string.as_bytes()[self.input_position] as char) <= 'z') |   // between a and z
                            ((self.input_string.as_bytes()[self.input_position] as char) >= '0') & ((self.input_string.as_bytes()[self.input_position] as char) <= '9') |   // between 0 and 9
                            ((self.input_string.as_bytes()[self.input_position] as char) == '\'') | ((self.input_string.as_bytes()[self.input_position] as char) == '\"') | // is ' or "
                            ((self.input_string.as_bytes()[self.input_position] as char) == '_') | ((self.input_string.as_bytes()[self.input_position] as char) == '.') {   // is _ or .

                            self.buffer_string.push(self.input_string.as_bytes()[self.input_position] as char);
                            self.input_position = self.input_position + 1;

                            if self.input_position == self.input_string.len() {
                                more = false;
                            }
                        }
                        else {
                            more = false;
                        }
                    }


                    if ((self.buffer_string.as_bytes()[0] as char) == '\'') & ((self.buffer_string.as_bytes()[self.buffer_string.len() - 1] as char) == '\'') {
                        self.buffer_string = self.buffer_string.replace("\'", "");
                        self.current_token = Token::LIT_CHAR(self.buffer_string.parse::<char>().unwrap());
                    }
                    else if ((self.buffer_string.as_bytes()[0] as char) == '\"') & ((self.buffer_string.as_bytes()[self.buffer_string.len() - 1] as char) == '\"'){
                        self.buffer_string = self.buffer_string.replace("\"", "");
                        self.current_token = Token::LIT_STRING(self.buffer_string.clone());
                    }
                    else {
                        self.current_state = 4;
                    }
                }
                if self.current_state == 3 {
                    self.current_state = -1;
                    self.buffer_string = "".to_string();
                }
            }

            else if self.current_state == 4 {
                let mut integer: bool = true;
                let mut floating: bool = true;

                for n in 0..self.buffer_string.len(){
                    if ((self.buffer_string.as_bytes()[n] as char) < '0') | ((self.buffer_string.as_bytes()[n] as char) > '9') {
                        integer = false;
                        if (self.buffer_string.as_bytes()[n] as char) != '.' {
                            floating = false;
                        }
                    }
                }

                if integer {
                    self.current_token = Token::LIT_I32(self.buffer_string.parse::<i32>().unwrap());
                }
                else if floating {
                    self.current_token = Token::LIT_F32(self.buffer_string.parse::<f32>().unwrap());
                }
                else {
                    self.current_state = 5;
                }

                if self.current_state == 4 {
                    self.current_state = -1;
                    self.buffer_string = "".to_string();
                }
            }

            else if self.current_state == 5 {
                match self.buffer_string.as_str() {
                    "not" => self.current_token = Token::OP_NOT,
                    "and" => self.current_token = Token::OP_AND,
                    "or" => self.current_token = Token::OP_OR,
                    "func" => self.current_token = Token::KW_FUNC,
                    "let" => self.current_token = Token::LET,
                    "if" => self.current_token = Token::IF,
                    "else" => self.current_token = Token::ELSE,
                    "while" => self.current_token = Token::WHILE,
                    "print" => self.current_token = Token::PRINT,
                    "return" => self.current_token = Token::RETURN,
                    "i32" => self.current_token = Token::TYPE_I32,
                    "int32" => self.current_token = Token::TYPE_I32,
                    "f32" => self.current_token = Token::TYPE_F32,
                    "float32" => self.current_token = Token::TYPE_F32,
                    "char" => self.current_token = Token::TYPE_CHAR,
                    "true" => self.current_token = Token::LIT_BOOL(true),
                    "false" => self.current_token = Token::LIT_BOOL(false),
                    _ => self.current_token = Token::ID(self.buffer_string.clone())
                }

                self.current_state = -1;
                self.buffer_string = "".to_string();
            }
        }
    }


}