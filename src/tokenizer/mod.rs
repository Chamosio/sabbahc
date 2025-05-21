mod helper;

pub use helper::keyword::KeywordType;
use helper::expression::ExpressionType;
use helper::expression::literal::LiteralType;

use std::process::exit;

#[derive(Debug, Clone)]
pub enum TokenType {
    Keyword(KeywordType),
    Expression(ExpressionType),
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub struct StatementAST {
    pub children: Vec<Token>,
}

impl StatementAST {
    pub fn new() -> Self {
        StatementAST {
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: Token) {
        self.children.push(child);
    }
}

pub struct Tokenizer {
    input: String,
    index: u64
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer {
            input,
            index: 0,
        }
    }
    fn peek(&self, index: u8) -> Option<char> {
        let peek_index = self.index + index as u64;
        if peek_index >= self.input.len() as u64 {
            return None;
        }
        let char = self.input.chars().nth(peek_index as usize).unwrap();
        Some(char)
    }
    fn consume(&mut self, index: u8) -> Option<char> {
        let mut consume_index = self.index + index as u64;
        if consume_index >= self.input.len() as u64 {
            return None;
        }
        let char = self.input.chars().nth(consume_index as usize).unwrap();
        consume_index += 1;
        self.index = consume_index;
        Some(char)
    }
    pub fn tokenize(&mut self) -> Vec<StatementAST> {
        let mut statements: Vec<StatementAST> = Vec::new();
        let mut tokens: StatementAST = StatementAST::new();
        let mut line: u16 = 1 as u16;
        while self.index < self.input.len() as u64 {
            let current_char = self.peek(0).unwrap();
            if current_char.is_whitespace() {
                if current_char == '\n' {
                    line += 1 as u16;
                }
                self.consume(0);
                continue
            }
            else if current_char.is_alphabetic() {
                let mut current = self.consume(0).unwrap().to_string();
                while self.peek(0).unwrap().is_alphanumeric() {
                    current.push(self.consume(0).unwrap());
                }
                match current.as_str() {
                    "exit" => {
                        tokens.add_child(Token {
                            token_type: TokenType::Keyword(KeywordType::Exit),
                            value: None,
                        });
                    },
                    _ => {
                        println!("Compile-time error on line {}: Unexpected text: {}", line, current);
                        exit(7);
                    }
                }
            }
            else if current_char.is_numeric() {
                let mut current = self.consume(0).unwrap().to_string();
                while self.peek(0).unwrap().is_numeric() {
                    current.push(self.consume(0).unwrap());
                }
                tokens.add_child(Token {
                    token_type: TokenType::Expression(ExpressionType::Literal(LiteralType::IntegerLiteral)),
                    value: Some(current),
                });
            }
            else if current_char == ';' {
                statements.push(StatementAST {
                    children: tokens.children.clone()
                });

                self.consume(0);
            }
        }
        statements
    }
}