use core::str::Chars;
use std::thread::current;

use crate::instruction;

pub struct Parser {
    program: String,
    tokens: Vec<Token>
}

#[derive(PartialEq)]
enum Token {
    Symbol(String),
    LeftParen,
    RightParen,
    Dot,
    Colon,
    Comma,
    Newline,
    Comment,
    EndOfFile,
}

impl Parser {
    pub fn new(p: String) -> Self {
        Self {
            program: p,
            tokens: vec![],
        }
    }

    fn is_newline(&self, c: char) -> bool {
        c == '\r' || c == '\n'
    }

    fn is_space(&self, c: char) -> bool {
        c == ' '
    }

    fn is_alpha(&self, c: char) -> bool {
        let c_code = c as u8;
        (c_code >= 65 && c_code <= 90) || (c_code >= 97 && c_code <= 122)
    }

    fn is_num(&self, c: char) -> bool {
        let c_code = c as u8;
        c_code >= 48 && c_code <= 57
    }

    fn is_punctuation(&self, c: char) -> bool {
        let c_code = c as u8;
        c_code == 36 || c_code == 39 || c_code == 46 || c_code == 95 || c_code == 96
    }

    fn tokenize(&mut self) {
        let mut program_iter = self.program.chars();
        
        let mut current_char = self.skip_spaces(program_iter.next(),&mut program_iter);

        loop {
            current_char = self.skip_spaces(current_char, &mut program_iter);

            match current_char {
                Some('\r') | Some('\n') => {
                    self.tokens.push(Token::Newline);
                    while self.is_newline(current_char.unwrap()) || self.is_space(current_char.unwrap()) {
                        current_char = program_iter.next();
                        if current_char == None {
                            self.tokens.push(Token::EndOfFile);
                            break;
                        }
                    }
                }
                Some('(') => {
                    self.tokens.push(Token::RightParen);
                    current_char = program_iter.next();
                }
                Some(')') => {
                    self.tokens.push(Token::LeftParen);
                    current_char = program_iter.next();
                }
                Some('.') => {
                    self.tokens.push(Token::Dot);
                    current_char = program_iter.next();
                }
                Some(':') => {
                    self.tokens.push(Token::Colon);
                    current_char = program_iter.next();
                }
                Some(',') => {
                    self.tokens.push(Token::Comma);
                    current_char = program_iter.next();
                }
                Some('#') => {
                    self.tokens.push(Token::Comment);
                    while !self.is_newline(current_char.unwrap()) {
                        current_char = program_iter.next();
                        if current_char == None {
                            self.tokens.push(Token::EndOfFile);
                            break;
                        }
                    }
                }
                None => {
                    self.tokens.push(Token::EndOfFile);
                    break;
                }
                _ => {
                    // Symbol
                    let mut symbol = String::from("");
                    while self.is_alpha(current_char.unwrap()) || self.is_num(current_char.unwrap()) || self.is_punctuation(current_char.unwrap()) {
                        symbol.push(current_char.unwrap());
                        current_char = program_iter.next();
                    }
                    self.tokens.push(Token::Symbol(symbol));
                }
            }
        }

        assert!(self.tokens.iter().last() == Some(&Token::EndOfFile));
    }

    fn skip_spaces(&self, mut current_char: Option<char>, chars: &mut Chars) -> Option<char> {
        while current_char == Some(' ') || current_char == Some('\t')  {
            current_char = chars.next();
        }

        return current_char;
    }

    pub fn parse(&mut self) {
        self.tokenize();

        let mut token_iter = self.tokens.iter().peekable();
        let mut current_token = token_iter.next();

        while current_token != Some(&Token::EndOfFile) {
            match token_iter.peek() {
                Some(Token::Colon) => self.label(),
                // TODO potentially prevent directive and instruction from being on the same line?
                Some(Token::Dot) => self.directive(),
                Some(Token::Symbol(..)) => self.instruction(),
                Some(Token::Comment) => current_token = token_iter.next(),
                None => todo!(),
                _ => todo!(),
            }
        }
    }

    fn label(&self) {

    }

    fn directive(&self) {

    }

    fn instruction(&self) {

    }
}
