use ari_errors;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)] // For equality comparisons
pub enum TokenType {
    LeftBracket, RightBracket, // Square Brackets
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
  
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
  
    // Literals.
    Identifier, String, Number,
  
    // Keywords.
    And, Class, Else, False, For, Fn, If, Null, Or,
    Print, Println, Return, Super, This, True, Let, While,
    Bai, // Quit

    // Loop keywords
    Break, // Quit while loop
    Continue, // Skip to the end of iteration

    
    // End of File
    Eof,

    // Empty placeholder
    None,
}

#[derive(Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String, // Either number or string
    pub line_number: usize,
    pub index: usize,
    pub source: String,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: &str, line_number: usize, index: usize, source: &str) -> Token {
        //println!("{:?}", token_type);
        Token {
            token_type,
            lexeme: lexeme.to_owned(), // Name of variables/keywords/arguments etc
            literal: literal.to_owned(), // Value such as string/number/bool etc
            line_number,
            index,
            source: source.to_owned(),
        }
    }
    pub fn none() -> Token{
        //println!("none");
        Token::new(TokenType::None, "", "", 0, 0, "")
    }

    pub fn print_error(&self, error: ari_errors::ErrorType) {
        ari_errors::print_error(error, &self.source, self.index + 1, self.line_number);
    }
    pub fn print_custom_error(&self, message: &str) {
        ari_errors::print_custom_error(message, &self.source, self.index + 1, self.line_number);
    }
}

impl Clone for Token { // Enables Token to be copied
    fn clone(&self) -> Token {
        Token {
            token_type: self.token_type,
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line_number: self.line_number,
            index: self.index,
            source: self.source.clone(),
        }
    }
}

