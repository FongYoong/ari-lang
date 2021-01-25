use crate::token;
use crate::ast;
use ari_errors;

pub struct Parser {

    tokens: Vec<token::Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec::<token::Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Box<ast::Statement>> {

        let mut statements = Vec::<Box<ast::Statement>>::new();
        while !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }
        return statements;
    }

    fn declaration(&mut self) -> Option<Box<ast::Statement>> {
        if self.check_next_tokens(vec![token::TokenType::Fn]) {
            return self.function_declaration("function");
        }
        if self.check_next_tokens(vec![token::TokenType::Let]) {
            return self.let_declaration();
        }
        return self.statement();
    }
    // Declaring new functions
    fn function_declaration(&mut self, func_type: &str) -> Option<Box<ast::Statement>> {
        // func_type can be 'function', 'class', and so on for error purposes.
        let error_type = match func_type {
            "function" => ari_errors::ErrorType::ExpectFunctionName,
            "class" => ari_errors::ErrorType::ExpectClassName,
            _ => panic!("function_declaration() does not implement {}", func_type)
        };
        let tok = self.consume(token::TokenType::Identifier, error_type); // Name of the function
        self.consume(token::TokenType::LeftParen, ari_errors::ErrorType::ExpectLeftParen);
        let mut arguments = Vec::<token::Token>::new(); // Arguments of the function
        if !self.check(token::TokenType::RightParen) {
            loop {
                if arguments.len()  >= 255 {
                    self.print_error(ari_errors::ErrorType::TooManyArguments);
                }
                arguments.push(self.consume(token::TokenType::Identifier, ari_errors::ErrorType::ExpectArgumentName));
                if !self.check_next_tokens(vec![token::TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(token::TokenType::RightParen, ari_errors::ErrorType::ExpectRightParen);
        self.consume(token::TokenType::LeftBrace, ari_errors::ErrorType::ExpectLeftBrace);
        let body = Some(Box::new(ast::Statement::new_block(self.block()))); // Body of the function
        return Some(Box::new(ast::Statement::new_function(body, tok, arguments)));
    }
    fn let_declaration(&mut self) -> Option<Box<ast::Statement>> {
        let tok = self.consume(token::TokenType::Identifier, ari_errors::ErrorType::ExpectVariableName);
        let initialisation = if self.check_next_tokens(vec![token::TokenType::Equal]) {
            self.expression()
        }
        else {
            Some(Box::new(ast::Expr::none()))
        };
        self.consume(token::TokenType::Semicolon, ari_errors::ErrorType::ExpectSemicolon);
        return Some(Box::new(ast::Statement::new_let(initialisation, tok)));
    }

    fn statement(&mut self) -> Option<Box<ast::Statement>> {
        let mut include_semicolon = true;
        if self.check_next_tokens(vec![token::TokenType::For]) {
            // For
            return self.for_statement();
        }
        let stmt = if self.check_next_tokens(vec![token::TokenType::If]) {
            // If
            include_semicolon = false;
            let (condition_expr, then_branch, else_branch) = self.if_statement();
            ast::Statement::new_if(condition_expr, then_branch, else_branch)
        }
        else if self.check_next_tokens(vec![token::TokenType::While]) {
            // While
            include_semicolon = false;
            let (condition_expr, body_branch) = self.while_statement();
            ast::Statement::new_while(condition_expr, body_branch)
        }
        else if self.check_next_tokens(vec![token::TokenType::Return]) {
            // Return from function
            let (tok, expr) = self.return_statement();
            ast::Statement::new_return(tok, expr)
        }
        else if self.check_next_tokens(vec![token::TokenType::Break]) {
            // Break loop
            ast::Statement::new_break()
        }
        else if self.check_next_tokens(vec![token::TokenType::Continue]) {
            // Continue loop
            ast::Statement::new_continue()
        }
        else if self.check_next_tokens(vec![token::TokenType::Print]) {
            // Print
            ast::Statement::new_print(self.expression())
        }
        else if self.check_next_tokens(vec![token::TokenType::Println]) {
            // Print with newline
            ast::Statement::new_println(self.expression())
        }
        else if self.check_next_tokens(vec![token::TokenType::Bai]) {
            // Exit interpreter
            ast::Statement::new_bai(self.expression())
        }
        else if self.check_next_tokens(vec![token::TokenType::LeftBrace]) {
            // Create block
            include_semicolon = false;
            ast::Statement::new_block(self.block())
        }
        else {
            // Create expression
            let expr = self.expression();
            // Check if variable exists prematurely, not sure if buggy because different from original
            let mut e = expr.clone().unwrap();
            if e.expr_type == ast::ExprType::Variable {
                e.evaluate_expr();
            }
            ast::Statement::new_expression(expr)
        };
        if include_semicolon {
            self.consume(token::TokenType::Semicolon, ari_errors::ErrorType::ExpectSemicolon);
        }
        return Some(Box::new(stmt));
    }

    fn if_statement(&mut self) -> (Option<Box<ast::Expr>>, Option<Box<ast::Statement>>, Option<Box<ast::Statement>>) {
        self.consume(token::TokenType::LeftParen, ari_errors::ErrorType::ExpectLeftParen);
        let condition_expr = self.expression();
        self.consume(token::TokenType::RightParen, ari_errors::ErrorType::ExpectRightParen);
        let then_branch = self.statement();
        let mut else_branch = None;
        if self.check_next_tokens(vec![token::TokenType::Else]) {
            else_branch = self.statement();
        }
        return (condition_expr, then_branch, else_branch);
    }

    fn while_statement(&mut self) -> (Option<Box<ast::Expr>>, Option<Box<ast::Statement>>) {
        self.consume(token::TokenType::LeftParen, ari_errors::ErrorType::ExpectLeftParen);
        let condition_expr = self.expression();
        self.consume(token::TokenType::RightParen, ari_errors::ErrorType::ExpectRightParen);
        let body_branch = self.statement();
        return (condition_expr, body_branch);
    }

    fn for_statement(&mut self) -> Option<Box<ast::Statement>> {
        self.consume(token::TokenType::LeftParen, ari_errors::ErrorType::ExpectLeftParen);

        // Initialisation
        let init_statement = if self.check_next_tokens(vec![token::TokenType::Semicolon]) {
            None
        }
        else if self.check_next_tokens(vec![token::TokenType::Let]) {
            self.let_declaration()
        }
        else {
            // expressionStatement()
            let statement = ast::Statement::new_expression(self.expression());
            self.consume(token::TokenType::Semicolon, ari_errors::ErrorType::ExpectSemicolon);
            Some(Box::new(statement))
        };

        // Conditional
        let condition_expr = if self.check_next_tokens(vec![token::TokenType::Semicolon]) {
            Some(Box::new(ast::Expr::literal(ast::Literal::bool(true))))
        }
        else {
            self.expression()
        };
        self.consume(token::TokenType::Semicolon, ari_errors::ErrorType::ExpectSemicolon);

        // Increment
        let increment_expr = if self.check_next_tokens(vec![token::TokenType::RightParen]) {
            Box::new(ast::Expr::none())
        }
        else {
            self.expression().unwrap()
        };
        self.consume(token::TokenType::RightParen, ari_errors::ErrorType::InvalidForLoop);
        

        // Put everything together 
        let mut body_branch = self.statement();
        if increment_expr.expr_type != ast::ExprType::None {
            let statements = vec![body_branch.unwrap(), Box::new(ast::Statement::new_expression(Some(increment_expr)))];
            body_branch = Some(Box::new(ast::Statement::new_block(statements)));
        }
        body_branch = Some(Box::new(ast::Statement::new_while(condition_expr, body_branch)));
        if !init_statement.is_none() {
            let statements = vec![init_statement.unwrap(), body_branch.unwrap()];
            body_branch = Some(Box::new(ast::Statement::new_block(statements)));
        }

        return body_branch;
    }

    fn return_statement(&mut self) -> (token::Token, Option<Box<ast::Expr>>) {
        let keyword = self.previous();
        let expr = if self.check(token::TokenType::Semicolon) {
            Some(Box::new(ast::Expr::none()))
        }
        else {
            self.expression()
        };
        return (keyword, expr);
    }

    fn block(&mut self) -> Vec<Box<ast::Statement>> {
        let mut statements = Vec::<Box<ast::Statement>>::new();
        while !self.check(token::TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }
        self.consume(token::TokenType::RightBrace, ari_errors::ErrorType::ExpectRightBrace);
        return statements;
    }

    fn expression(&mut self) -> Option<Box<ast::Expr>> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Option<Box<ast::Expr>> {     
        let expr = self.or().unwrap();
        if self.check_next_tokens(vec![token::TokenType::Equal]) {
            if expr.expr_type == ast::ExprType::Variable {
                //println!("wut, normal");
                //let equals_token = self.previous(); // Uselesss
                let value_expr = self.assignment().unwrap();
                let name_token = expr.operator.clone();
                return Some(Box::new(ast::Expr::assign(Some(value_expr), name_token)));
            }
            else if expr.expr_type == ast::ExprType::ArrayAccess {
                //println!("hooh");
                //let equals_token = self.previous();
                let ref_token = expr.left.unwrap().operator.clone();
                let index_expr = expr.right.clone();
                let value_expr = self.or().unwrap();
                return Some(Box::new(ast::Expr::array_assign(index_expr, Some(value_expr), ref_token)));
            }
            self.print_error(ari_errors::ErrorType::InvalidAssignment);
        }
        return Some(expr);
    }

    fn or(&mut self) -> Option<Box<ast::Expr>> {
        let mut expr = self.and();
        while self.check_next_tokens(vec![token::TokenType::Or]) {
            let operator = self.previous();
            let right = self.and();
            expr = Some(Box::new(ast::Expr::logical(expr, right, operator)));
        }
        return expr;
    }

    fn and(&mut self) -> Option<Box<ast::Expr>> {
        let mut expr = self.equality();
        while self.check_next_tokens(vec![token::TokenType::And]) {
            let operator = self.previous();
            let right = self.equality();
            expr = Some(Box::new(ast::Expr::logical(expr, right, operator)));
        }
        return expr;
    }
 
    fn equality(&mut self) -> Option<Box<ast::Expr>> {
        let mut expr = self.comparison();
        while self.check_next_tokens(vec![token::TokenType::BangEqual, token::TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Some(Box::new(ast::Expr::binary(expr, right, operator)));
        }
        return expr;
    }

    fn comparison(&mut self) -> Option<Box<ast::Expr>>{
        let mut expr = self.term();
        while self.check_next_tokens(vec![token::TokenType::Greater, token::TokenType::GreaterEqual, token::TokenType::Less, token::TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Some(Box::new(ast::Expr::binary(expr, right, operator)));
        }
        return expr
    }

    fn term(&mut self) -> Option<Box<ast::Expr>>{
        let mut expr = self.factor();
        while self.check_next_tokens(vec![token::TokenType::Minus, token::TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Some(Box::new(ast::Expr::binary(expr, right, operator)));
        }
        return expr;
    }

    fn factor(&mut self) -> Option<Box<ast::Expr>>{
        //let mut expr = self.unary();
        let mut expr = self.array_creation();
        while self.check_next_tokens(vec![token::TokenType::Slash, token::TokenType::Star]) {
            let operator = self.previous();
            //let right = self.unary();
            let right = self.array_creation();
            expr = Some(Box::new(ast::Expr::binary(expr, right, operator)));
        }
        return expr;
    }
    
    // Array creation
    fn array_creation(&mut self) -> Option<Box<ast::Expr>>{
        if self.check_next_tokens(vec![token::TokenType::LeftBracket]) {
            let mut array_values = Vec::<Box<ast::Expr>>::new();
            if !self.check(token::TokenType::RightBracket) {
                loop {
                    array_values.push(self.expression().unwrap());
                    if !self.check_next_tokens(vec![token::TokenType::Comma]) {
                        break;
                    }
                }
            }
            let parentheses = self.consume(token::TokenType::RightBracket, ari_errors::ErrorType::ExpectRightBracket);
            return Some(Box::new(ast::Expr::array_creation(parentheses, array_values)));
        }
        return self.unary();
    }


    fn unary(&mut self) -> Option<Box<ast::Expr>>{
        if self.check_next_tokens(vec![token::TokenType::Bang, token::TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Some(Box::new(ast::Expr::unary(right, operator)));
        }
        //return self.call();
        return self.array_access();
    }

    // Array access
    fn array_access(&mut self) -> Option<Box<ast::Expr>>{
        let expr = self.primary(); //  Array reference
        if self.check_next_tokens(vec![token::TokenType::LeftBracket]) {
            if self.check(token::TokenType::RightBracket) {
                self.print_error(ari_errors::ErrorType::NoArrayAccessIndex);
                panic!();
            }
            else {
                let index_expr = self.expression(); // Array index expression
                if self.check_next_tokens(vec![token::TokenType::Comma]) {
                    self.previous().print_error(ari_errors::ErrorType::ArrayAccessComma);
                }
                let brackets = self.consume(token::TokenType::RightBracket, ari_errors::ErrorType::ExpectRightBracket);
                return Some(Box::new(ast::Expr::array_access(expr, index_expr, brackets)));
            }
        }
        else {
            return self.call(expr);
        }
    }

    // Function calling/invocation
    fn call(&mut self, mut expr: Option<Box<ast::Expr>>) -> Option<Box<ast::Expr>>{
        //let mut expr = self.primary();
        loop {
            if self.check_next_tokens(vec![token::TokenType::LeftParen]) {
                expr = self.finish_call(expr);
            }
            else {
                break;
            }
        }
        return expr;
    }
    fn finish_call(&mut self, callee: Option<Box<ast::Expr>>) -> Option<Box<ast::Expr>>{
        let mut arguments = Vec::<Box<ast::Expr>>::new();
        if !self.check(token::TokenType::RightParen) {
            loop {
                if arguments.len()  >= 255 {
                    self.print_error(ari_errors::ErrorType::TooManyArguments);
                }
                arguments.push(self.expression().unwrap());
                if !self.check_next_tokens(vec![token::TokenType::Comma]) {
                    break;
                }
            }
        }
        let parentheses = self.consume(token::TokenType::RightParen, ari_errors::ErrorType::ExpectRightParen);
        return Some(Box::new(ast::Expr::call(callee, parentheses, arguments)));
    }

    fn primary(&mut self) -> Option<Box<ast::Expr>>{
        //println!("->{:?}", self.peek().token_type);
        if self.check_next_tokens(vec![token::TokenType::Null]){
            return Some(Box::new(ast::Expr::literal(ast::Literal::null())));
        }
        if self.check_next_tokens(vec![token::TokenType::False]){
            return Some(Box::new(ast::Expr::literal(ast::Literal::bool(false))));
        }
        if self.check_next_tokens(vec![token::TokenType::True]) {
            return Some(Box::new(ast::Expr::literal(ast::Literal::bool(true))));
        }
        if self.check_next_tokens(vec![token::TokenType::Number]) {
            return Some(Box::new(ast::Expr::literal(ast::Literal::number(self.previous().literal))));
        }
        if self.check_next_tokens(vec![token::TokenType::String]) {
            return Some(Box::new(ast::Expr::literal(ast::Literal::string(self.previous().literal))));
        }
        if self.check_next_tokens(vec![token::TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(token::TokenType::RightParen, ari_errors::ErrorType::ExpectRightParen);
            return Some(Box::new(ast::Expr::grouping(expr)));
        }
        if self.check_next_tokens(vec![token::TokenType::Identifier]) {
            return Some(Box::new(ast::Expr::variable(self.previous())));
        }
        self.print_error(ari_errors::ErrorType::ExpectExpression);
        None
    }

    fn consume(&mut self, token_type: token::TokenType, error_type: ari_errors::ErrorType) -> token::Token {
        if self.check(token_type){
            return self.advance();
        }
        self.print_error(error_type);
        token::Token::none()
    }

    fn check_next_tokens(&mut self, expected : Vec::<token::TokenType>) -> bool{
        for tok in expected{
            if self.check(tok) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn check(&mut self, token_type : token::TokenType) -> bool{
        if self.is_at_end(){
            return false;
        }
        return self.peek().token_type == token_type;
    }
    fn advance(&mut self) -> token::Token {
        if !self.is_at_end(){
            self.current +=1;
        }
        return self.previous();
    }
    fn peek(&mut self) -> token::Token {
        return self.tokens[self.current].clone();
    }
    fn previous(&mut self) -> token::Token {
        return self.tokens[self.current - 1].clone();
    }
    fn is_at_end(&mut self)-> bool{
        return self.peek().token_type == token::TokenType::Eof;
    }
    fn print_error(&mut self, error: ari_errors::ErrorType){
        let tok = self.peek();
        tok.print_error(error);
    }
}