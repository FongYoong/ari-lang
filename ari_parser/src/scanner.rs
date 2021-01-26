use crate::token;
use ari_errors;

#[derive(Debug)]
pub struct Scanner <'a>{
    source: &'a str,
    pub tokens: Vec<token::Token>,
    start: usize,
    current: usize,
    line_index: usize,
    line_number: usize,
}
impl Scanner <'_>{
    pub fn new<'a>(source: &'a str, line_number: usize) -> Scanner<'a> {
        Scanner {
            source: source,
            tokens: Vec::<token::Token>::new(),
            start: 0,
            current: 0,
            line_index: 0,
            line_number,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<token::Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let line = self.get_current_line();
        self.tokens.push(token::Token::new(token::TokenType::Eof, "", "", self.line_number, self.current - self.line_index, &line));
        return self.tokens.clone();
    }

    fn scan_token(&mut self){
        let c = self.advance();
        let next_c_bool = self.check_next_token(match c {
            '!' => '=',
            '=' => '=',
            '<' => '=',
            '>' => '=',
            '/' => '/',
            _ => '\0'
        });
        match c {
            '[' => {self.add_token(token::TokenType::LeftBracket, "");},
            ']' => {self.add_token(token::TokenType::RightBracket, "");},
            '(' => {self.add_token(token::TokenType::LeftParen, "");},
            ')' => {self.add_token(token::TokenType::RightParen, "");},
            '{' => {self.add_token(token::TokenType::LeftBrace, "");},
            '}' => {self.add_token(token::TokenType::RightBrace, "");},
            ',' => {self.add_token(token::TokenType::Comma, "");},
            '.' => {self.add_token(token::TokenType::Dot, "");},
            '-' => {self.add_token(token::TokenType::Minus, "");},
            '+' => {self.add_token(token::TokenType::Plus, "");},
            ';' => {self.add_token(token::TokenType::Semicolon, "");},
            '*' => {self.add_token(token::TokenType::Star, "");},
            '!' => {self.add_token(
                if next_c_bool {token::TokenType::BangEqual}
                else {token::TokenType::Bang}
                , "");},
            '=' => {self.add_token(
                if next_c_bool {token::TokenType::EqualEqual}
                else {token::TokenType::Equal}
                , "");},
            '<' => {self.add_token(
                if next_c_bool {token::TokenType::LessEqual}
                else {token::TokenType::Less}
                , "");},
            '>' => {self.add_token(
                if next_c_bool {token::TokenType::GreaterEqual}
                else {token::TokenType::Greater}
                , "");},
            '/' => {
                if next_c_bool {
                    while self.peek() != '\n' && !self.is_at_end(){
                        self.advance();
                    }
                }
                else {
                    self.add_token(token::TokenType::Slash, "");
                }},
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => {
                self.advance_line();
            },
            '"' => {
                self.consume_string_lexeme();
            },
            _ => {
                if c.is_numeric() {
                    self.consume_number_lexeme();
                }  
                else if self.is_alpha(c) || c == '_' {
                    self.consume_identifier();
                }
                else {
                    self.print_error(ari_errors::ErrorType::UnknownToken);
                }
            },

        }
    }
    fn get_char(&mut self, index: usize) -> char{
        return self.source.chars().nth(index).unwrap();
    }
    fn get_current_line(&mut self) -> String {
        return self.source[self.line_index .. self.current].to_owned();
    }
    fn advance_line(&mut self) {
        self.line_index = self.current;
        self.line_number += 1;
    }
    fn advance(&mut self) -> char{
        self.current+=1;
        return self.get_char(self.current - 1);
    }
    fn add_token(&mut self, token_type: token::TokenType, literal: &str){
        let text = &self.source[self.start..self.current];
        let line = self.get_current_line();
        self.tokens.push(token::Token::new(token_type, text, literal, self.line_number, self.current - self.line_index, &line));
        //println!("Line {}\n", self.line_number);
    }
    fn check_next_token(&mut self, expected : char) -> bool{
        if self.is_at_end() || (self.get_char(self.current) != expected) {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn peek(&mut self) -> char{
        if self.is_at_end(){
            return '\0';
        }
        return self.get_char(self.current);
    }
    fn peek_next(&mut self) -> char{
        if self.current + 1 >= self.source.len(){
            return '\0';
        }
        return self.get_char(self.current + 1);
    }
    fn consume_string_lexeme(&mut self){
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n'{
                self.line_number += 1;
            }
            self.advance();
        }
        if self.is_at_end(){
            self.print_error(ari_errors::ErrorType::ConsumeStringLexeme);
        }
        self.advance();
        self.add_token(token::TokenType::String, &self.source[self.start + 1 .. self.current - 1].to_owned());
    }
    fn consume_number_lexeme(&mut self){
        while self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance(); // Consume the '.'
            while self.peek().is_numeric() {
                self.advance();
            }
        }
        self.add_token(token::TokenType::Number, &self.source[self.start .. self.current].to_owned());
    }
    fn consume_identifier(&mut self){
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start .. self.current].to_owned();
        let keyword_type = match self.get_reserved_keyword(text){
            Some(keyword) => {keyword},
            None => {token::TokenType::Identifier}
        };
        self.add_token(keyword_type, "");
    }

    fn get_reserved_keyword(&mut self, keyword : &str) -> Option<token::TokenType>{
        match keyword{
            "and" => Some(token::TokenType::And),
            "class" => Some(token::TokenType::Class),
            "else" => Some(token::TokenType::Else),
            "false" => Some(token::TokenType::False),
            "for" => Some(token::TokenType::For),
            "fn" => Some(token::TokenType::Fn), // Declare function
            "if" => Some(token::TokenType::If),
            "null" => Some(token::TokenType::Null),
            "or" => Some(token::TokenType::Or),
            "print" => Some(token::TokenType::Print),
            "println" => Some(token::TokenType::Println),
            "return" => Some(token::TokenType::Return),
            "super" => Some(token::TokenType::Super),
            "this" => Some(token::TokenType::This),
            "true" => Some(token::TokenType::True),
            "let" => Some(token::TokenType::Let),
            "while" => Some(token::TokenType::While),
            "bai" => Some(token::TokenType::Bai),
            "break" => Some(token::TokenType::Break),
            "continue" => Some(token::TokenType::Continue),
            _ => None
        }
    }
    fn is_at_end(&mut self)-> bool{
        return self.current >= self.source.chars().count();
    }
    fn is_alpha(&mut self, c : char) -> bool{
        c.is_alphabetic() || c == '_'
    }

    fn print_error(&mut self, error: ari_errors::ErrorType){
        ari_errors::print_error(error, &self.get_current_line(), self.current - self.line_index, self.line_number)
    }
}