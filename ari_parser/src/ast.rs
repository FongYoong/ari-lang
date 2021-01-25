use crate::token;
use crate::function as func;
use crate::environment::ENV;
use ari_errors;
use rayon::prelude::*; // For array operations/fast parallelism

///////////////////////////////////////////
// Literals
///////////////////////////////////////////

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)] // For equality comparisons
pub enum LiteralType {
    // 'value'
    None,
    Number,
    String,
    Bool,
    Null,

    Array,
    
    // function
    Function,

    // Loop commands, uses no fields
    Break,
    Continue,
    
}

pub struct Literal {
    literal_type : LiteralType,
    pub value : String,

    // Array
    pub array_values: Vec<Literal>,

    // Function
    pub function: Option<func::Function>,
    pub is_return: bool, // Must be manually modified
}

impl Clone for Literal { // Enables Literal to be copied
    fn clone(&self) -> Literal {
        Literal {
            literal_type: self.literal_type,
            value: self.value.clone(),
            array_values: self.array_values.clone(),
            function: self.function.clone(),
            is_return: self.is_return,
        }
    }
}

impl Literal {
    pub fn new(literal_type : LiteralType, value: String, array_values: Vec<Literal>, function: Option<func::Function>, is_return: bool) -> Literal {
        Literal {
            literal_type,
            value,
            array_values,
            function,
            is_return,
        }
    }

    // Values
    pub fn new_value(literal_type : LiteralType, value: String) -> Literal {
        Literal::new(literal_type, value, Vec::<Literal>::new(), None, false)
    }
    pub fn none() -> Literal {
        Literal::new_value(LiteralType::None, "".to_string())
    }
    pub fn number(value: String) -> Literal {
        Literal::new_value(LiteralType::Number, value)
    }
    pub fn string(value: String) -> Literal {
        Literal::new_value(LiteralType::String, value)
    }
    pub fn bool(value: bool) -> Literal {
        Literal::new_value(LiteralType::Bool, String::from(if value {"true"} else {"false"}))
    }
    pub fn null() -> Literal {
        Literal::new_value(LiteralType::Null, "null".to_string())
    }

    // Array
    pub fn new_array(array_values: Vec<Literal>) -> Literal {
        Literal::new(LiteralType::Array, "".to_string(), array_values, None, false)
    }

    // Function
    pub fn new_function(function: func::Function) -> Literal {
        Literal::new(LiteralType::Function, "".to_string(), Vec::<Literal>::new(), Some(function), false)
    }

    // Loop commands
    pub fn new_break() -> Literal {
        Literal::new_value(LiteralType::Break, "".to_string())
    }
    pub fn new_continue() -> Literal {
        Literal::new_value(LiteralType::Continue, "".to_string())
    }
}

///////////////////////////////////////////
// Statements
///////////////////////////////////////////

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum StatementType {
    Block, // 'statements'
    Expression, // 'expr'

    // Function
    Function, // 'then_branch', 'token_name', 'tokens'
    Return, // 'token_name', 'expr'

    // Control Flow
    If, // 'then_branch', 'else_branch', 'expr'
    While, // 'expr' (condition), 'then_branch' (body)


    // Special
    Let, // 'expr'/value and 'token_name'/variable name
    Print, // 'expr'
    Println, // 'expr'
    Bai, // 'expr'
    
    Break, // Nothing
    Continue, // Nothing
}

pub struct Statement {
    pub statement_type: StatementType,
    pub statements: Vec<Box<Statement>>,
    pub then_branch: Option<Box<Statement>>,
    pub else_branch: Option<Box<Statement>>,
    pub expr: Option<Box<Expr>>,
    pub token_name: token::Token,
    pub tokens: Vec<token::Token>,
}

impl Clone for Box<Statement> {
    fn clone(&self) -> Box<Statement> {
        Box::new(Statement::new(self.statement_type, self.statements.clone(),
        self.then_branch.clone(), self.else_branch.clone(),
        self.expr.clone(), self.token_name.clone(), self.tokens.clone()))
    }
}

impl Statement {
    pub fn new(statement_type : StatementType, statements: Vec<Box<Statement>>,
                then_branch: Option<Box<Statement>>, else_branch: Option<Box<Statement>>,
                expr: Option<Box<Expr>>, token_name: token::Token, tokens: Vec<token::Token>) -> Statement {
        Statement {
            statement_type,
            statements,
            then_branch,
            else_branch,
            expr,
            token_name,
            tokens,

        }
    }
    pub fn new_block(statements: Vec<Box<Statement>>) -> Statement {
        Statement::new(StatementType::Block, statements, None, None, None, token::Token::none(), Vec::<token::Token>::new())
    }
    pub fn new_break() -> Statement {
        Statement::new(StatementType::Break, Vec::<Box<Statement>>::new(), None, None, None, token::Token::none(), Vec::<token::Token>::new())
    }
    pub fn new_continue() -> Statement {
        Statement::new(StatementType::Continue, Vec::<Box<Statement>>::new(), None, None, None, token::Token::none(), Vec::<token::Token>::new())
    }
    pub fn new_expression(expr : Option<Box<Expr>>) -> Statement {
        Statement::new(StatementType::Expression, Vec::<Box<Statement>>::new(), None, None, expr, token::Token::none(), Vec::<token::Token>::new())
    }

    // For delcaring user-defined function
    pub fn new_function(then_branch: Option<Box<Statement>>, token_name: token::Token, tokens: Vec<token::Token>) -> Statement {
        Statement::new(StatementType::Function, Vec::<Box<Statement>>::new(), then_branch, None, None, token_name, tokens)
    }
    // Function return
    pub fn new_return(token_name: token::Token, expr: Option<Box<Expr>>) -> Statement {
        Statement::new(StatementType::Return, Vec::<Box<Statement>>::new(), None, None, expr, token_name, Vec::<token::Token>::new())
    }

    // Conditional
    pub fn new_if(condition_expr : Option<Box<Expr>>, then_branch : Option<Box<Statement>>,  else_branch : Option<Box<Statement>>) -> Statement {
        Statement::new(StatementType::If, Vec::<Box<Statement>>::new(), then_branch, else_branch, condition_expr, token::Token::none(), Vec::<token::Token>::new())
    }
    pub fn new_while(condition_expr : Option<Box<Expr>>, body : Option<Box<Statement>>) -> Statement {
        Statement::new(StatementType::While, Vec::<Box<Statement>>::new(), body, None, condition_expr, token::Token::none(), Vec::<token::Token>::new())
    }

    // Special
    pub fn new_print(expr : Option<Box<Expr>>) -> Statement {
        Statement::new(StatementType::Print, Vec::<Box<Statement>>::new(), None, None, expr, token::Token::none(), Vec::<token::Token>::new())
    }
    pub fn new_println(expr : Option<Box<Expr>>) -> Statement {
        Statement::new(StatementType::Println, Vec::<Box<Statement>>::new(), None, None, expr, token::Token::none(), Vec::<token::Token>::new())
    }
    pub fn new_let(expr : Option<Box<Expr>>, token_name : token::Token) -> Statement {
        Statement::new(StatementType::Let, Vec::<Box<Statement>>::new(), None, None, expr, token_name, Vec::<token::Token>::new())
    }
    pub fn new_bai(expr : Option<Box<Expr>>) -> Statement {
        Statement::new(StatementType::Bai, Vec::<Box<Statement>>::new(), None, None, expr, token::Token::none(), Vec::<token::Token>::new())
    }

    pub fn print(&mut self, newline: bool) {
        let max_display = 5; // Maximum elements to display
        let literal = self.expr.as_mut().unwrap().evaluate_expr();
        if literal.literal_type == LiteralType::Array {
            let length = literal.array_values.len();
            print!("{:?}({}) => [", literal.array_values.get(0).unwrap().literal_type, length);
            let mut index = 0;
            for value in literal.array_values {
                if index >= max_display {
                    break;
                }
                print!("{}", value.value);
                if index != length - 1 {
                    print!(",");
                }
                index += 1;
            }
            if length >= max_display + 1 {
                print!(" ...");
            }
            print!("]");
            if newline {
                print!("\n");
            }
        }
        else {
            println!("{}", literal.value);
        }
    }

    pub fn evaluate_statement(&mut self) -> Literal {
        match self.statement_type {
            StatementType::Function => {
                // Declare user-defined function
                let closure_env = ENV.lock().unwrap().get_env().clone();
                let new_user_function = func::Function::new_user(self.tokens.clone(), self.then_branch.clone(), closure_env, self.token_name.clone());
                ENV.lock().unwrap().get_env().define(self.token_name.lexeme.to_owned(), Literal::new_function(new_user_function));
                return Literal::none();
            },
            StatementType::Return => {
                // Returns from enclosing function
                let mut literal = self.expr.as_mut().unwrap().evaluate_expr();
                literal.is_return = true;
                return literal;
            },

            StatementType::Block => {
                ENV.lock().unwrap().create_env();
                let mut continue_condition = false;
                let mut result = Literal::none();
                for s in &mut self.statements {
                    let literal = s.evaluate_statement();
                    if literal.literal_type == LiteralType::Break || literal.is_return {
                        result = literal;
                        break;
                    }
                    else if literal.literal_type == LiteralType::Continue {
                        continue_condition = true;
                        break;
                    }
                }
                ENV.lock().unwrap().destroy_env();
                if continue_condition {
                    return Literal::new_continue();
                }
                return result;
            },
            StatementType::Expression => {
                return self.expr.as_mut().unwrap().evaluate_expr();
            },

            // Conditional
            StatementType::If => {
                let expr = self.expr.as_mut().unwrap();
                let condition_literal = expr.evaluate_expr();
                if !Expr::is_truthy(&condition_literal) {
                    expr.print_custom_error(&format!("'If' conditional cannot be applied to {:?}", condition_literal.literal_type));
                }
                if expr.string_to_bool(&condition_literal) {
                    let result = self.then_branch.as_mut().unwrap().evaluate_statement();
                    if result.literal_type == LiteralType::Break || result.literal_type == LiteralType::Continue || result.is_return {
                        return result;
                    }
                }
                else {
                    match self.else_branch.as_mut() {
                        Some(else_statement) => {
                            let result = else_statement.evaluate_statement();
                            if result.literal_type == LiteralType::Break || result.literal_type == LiteralType::Continue || result.is_return {
                                return result;
                            }
                        },
                        None => {}
                    };
                }
                return Literal::none();
            },
            StatementType::While => {
                loop {
                    let expr = self.expr.as_mut().unwrap();
                    let condition_literal = expr.evaluate_expr();
                    if !Expr::is_truthy(&condition_literal) {
                        expr.print_custom_error(&format!("'While' conditional cannot be applied to {:?}", condition_literal.literal_type));
                    }
                    // Evaluate 'then' branch
                    if expr.string_to_bool(&condition_literal) {
                        let result = self.then_branch.as_mut().unwrap().evaluate_statement();
                        if result.literal_type == LiteralType::Break {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }
                return Literal::none();
            },

            // Loop keywords
            StatementType::Break => {
                return Literal::new_break();
            },
            StatementType::Continue => {
                return Literal::new_continue();
            },

            // Special
            StatementType::Print => {
                self.print(false);
                return Literal::none();
            },
            StatementType::Println => {
                self.print(true);
                return Literal::none();
            },
            StatementType::Let => {
                let expr = self.expr.as_mut().unwrap();
                if expr.expr_type == ExprType::None {
                    self.print_error(ari_errors::ErrorType::InvalidVariableDefinition);
                    return Literal::none();
                }
                let mut literal = expr.evaluate_expr();
                if literal.literal_type == LiteralType::Function {
                    literal.function.as_mut().unwrap().variable_token = self.token_name.clone();
                }
                ENV.lock().unwrap().get_env().define(self.token_name.lexeme.to_owned(), literal.clone());
                return literal;
            },
            StatementType::Bai => {
                let literal = self.expr.as_mut().unwrap().evaluate_expr();
                let value = match literal.value.as_str() {
                    "0" => "",
                    "1" => "\nPoof",
                    "2" => "\nI lub Ariana",
                    "3" => "\nBye Nigga",
                    "4" => "\nStop messing around with this function",
                    _ => literal.value.as_str()
                };
                ari_errors::print_yellow(&value, true, false);
                ari_errors::exit();
                return Literal::none();
            },
            _ => {
                return Literal::none();
            }
        }
    }
    fn print_error(&mut self, error: ari_errors::ErrorType){
        self.token_name.print_error(error);
    }
    fn print_custom_error(&mut self, message: &str){
        self.token_name.print_custom_error(message);
    }
}

///////////////////////////////////////////
// Expressions
///////////////////////////////////////////

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum ExprType {
    Binary, // Uses 'left', 'right', 'operator'
    Logical, // (or, and) // Uses 'left', 'right', 'operator'
    ArrayCreation, // Uses 'arguments' for values
    ArrayAccess, // Uses 'left' for array reference, 'right' for array index, 'operator' for error purposes
    Unary, // Uses 'right' and 'operator' field
    Literal, // Uses 'literal' field
    Grouping, // Uses 'right' field
    
    Variable, // Uses 'operator' field to represent token
    Assign, // Uses 'operator' field to represent variable token, 'right' field for expression
    ArrayAssign, // Uses 'operator' field to represent variable token, 'left' field for index, 'right' field for expression

    Call, // Uses 'right' (callee), 'operator' (closing parentheses), 'arguments'

    // Empty placeholder
    None,
}

pub struct Expr {
    pub expr_type: ExprType,
    pub arguments: Vec<Box<Expr>>,
    pub left: Option<Box<Expr>>,
    pub right: Option<Box<Expr>>,
    pub operator: token::Token,
    pub literal: Literal,
}

impl Clone for Box<Expr> {
    fn clone(&self) -> Box<Expr> {
        Box::new(Expr::new(self.expr_type, self.arguments.clone(),
        self.left.clone(), self.right.clone(),
        self.operator.clone(), self.literal.clone()))
    }
}

impl Expr {
    pub fn new(expr_type : ExprType, arguments: Vec<Box<Expr>>, left : Option<Box<Expr>>, right : Option<Box<Expr>>, operator : token::Token, literal : Literal) -> Expr {
        Expr {
            expr_type,
            arguments,
            left,
            right,
            operator,
            literal
        }
    }
    pub fn none() -> Expr {
        Expr::new(ExprType::None, Vec::<Box<Expr>>::new(), None, None, token::Token::none(), Literal::none())
    }
    pub fn binary(left : Option<Box<Expr>>, right : Option<Box<Expr>>, operator : token::Token) -> Expr {
        Expr::new(ExprType::Binary, Vec::<Box<Expr>>::new(), left, right, operator, Literal::none())
    }
    pub fn logical(left : Option<Box<Expr>>, right : Option<Box<Expr>>, operator : token::Token) -> Expr {
        Expr::new(ExprType::Logical, Vec::<Box<Expr>>::new(), left, right, operator, Literal::none())
    }
    pub fn literal(literal : Literal) -> Expr {
        Expr::new(ExprType::Literal, Vec::<Box<Expr>>::new(), None, None, token::Token::none(), literal)
    }
    pub fn unary(right : Option<Box<Expr>>, operator : token::Token) -> Expr {
        Expr::new(ExprType::Unary, Vec::<Box<Expr>>::new(), None, right, operator, Literal::none())
    }
    pub fn grouping(right : Option<Box<Expr>>) -> Expr {
        Expr::new(ExprType::Grouping, Vec::<Box<Expr>>::new(), None, right, token::Token::none(), Literal::none())
    }
    pub fn variable(tok : token::Token) -> Expr {
        Expr::new(ExprType::Variable, Vec::<Box<Expr>>::new(), None, None, tok, Literal::none())
    }
    pub fn assign(right : Option<Box<Expr>>, tok : token::Token) -> Expr {
        Expr::new(ExprType::Assign, Vec::<Box<Expr>>::new(), None, right, tok, Literal::none())
    }
    pub fn array_assign(left : Option<Box<Expr>>, right : Option<Box<Expr>>, tok : token::Token) -> Expr {
        Expr::new(ExprType::ArrayAssign, Vec::<Box<Expr>>::new(), left, right, tok, Literal::none())
    }

    // Array
    pub fn array_creation(tok : token::Token, array_values: Vec<Box<Expr>>) -> Expr {
        Expr::new(ExprType::ArrayCreation, array_values, None, None, tok, Literal::none())
    }
    pub fn array_access(left : Option<Box<Expr>>, right : Option<Box<Expr>>, tok : token::Token) -> Expr {
        Expr::new(ExprType::ArrayAccess, Vec::<Box<Expr>>::new(), left, right, tok, Literal::none())
    }

    // Function
    pub fn call(right : Option<Box<Expr>>, tok : token::Token, arguments: Vec<Box<Expr>>) -> Expr {
        Expr::new(ExprType::Call, arguments, None, right, tok, Literal::none())
    }

    // Helper functions
    pub fn is_valid_arithmetic(left_type : LiteralType, right_type : LiteralType) -> bool{
        return (left_type == right_type) && (left_type == LiteralType::Number || left_type == LiteralType::Array);
    }
    pub fn add_or_concat(left_type : LiteralType, right_type : LiteralType) -> Result<bool, ()>{
        let left_is_number = left_type == LiteralType::Number;
        let left_is_string = left_type == LiteralType::String;
        let right_is_number = right_type == LiteralType::Number;
        let right_is_string = right_type == LiteralType::String;
        let mut mixed_concat = false; // Represents whether to concat string and number and vice versa
        if left_type != right_type {
            if (left_is_string && right_is_number) || (left_is_number && right_is_string) {
                mixed_concat = true;
            }
            else {
                // Unsupported types
                return Err(());
            }
        }
        return Ok(mixed_concat);
    }
    pub fn add(left : &Literal, right : &Literal, string_concat: bool) -> Literal {
        if string_concat {
            // Concatenate strings
            let result = left.value.to_owned() + &right.value;
            return Literal::new_value(LiteralType::String, result.to_string());
        }
        else {
            let result = Expr::string_to_float(&left) + Expr::string_to_float(&right);
            return Literal::new_value(LiteralType::Number, result.to_string());
        }
    }
    pub fn is_truthy(literal : &Literal) -> bool{
        return (literal.literal_type == LiteralType::Bool) || (literal.literal_type == LiteralType::Null)
    }
    pub fn string_to_bool(&mut self, literal : &Literal) -> bool {
        // Converts bool and null
        // &mut self is included for the purpose of tracking down the error location
        let result = match literal.value.as_str() {
            "true" => {
                true
            },
            "false" => {
                false
            }
            "null" => {
                false
            }
            _ => {
                self.print_custom_error("Boolean operation only handles 'true', 'false', 'null' values.");
                panic!();
            }
        };
        return result;
    }
    pub fn is_equal(&mut self, op_name: &str, left_type: LiteralType, right_type: LiteralType, left_string: &str, right_string: &str) -> bool {
        // &mut self is included for the purpose of tracking down the error location
        if left_type != right_type {
            self.print_custom_error(&format!("{} cannot be applied to {:?} and {:?}", op_name, left_type, right_type));
        }
        match left_type {
            LiteralType::Number => {
                return left_string.parse::<f32>().unwrap() == right_string.parse::<f32>().unwrap();
            },
            LiteralType::String | LiteralType::Bool | LiteralType::Null => {
                return left_string == right_string;
            },
            //////// Cover classes here onwards
            // 
            ////////
            _ => {
                self.print_custom_error(&format!("{} cannot be applied to {:?} and {:?}", op_name, left_type, right_type));
                panic!();
            }
        };
    }

    pub fn string_to_float(literal: &Literal) -> f32 {
        return literal.value.parse::<f32>().unwrap();
    }

    pub fn divide(left: &Literal, right: &Literal) -> Result<f32, ()> {
        let right_value = Expr::string_to_float(&right);
        if right_value as i32 == 0 {
           return Err(());
        }
        return Ok(Expr::string_to_float(&left) / right_value);
    }

    // Evaluate expression
    pub fn evaluate_expr(&mut self) -> Literal {
        match self.expr_type {
            ExprType::Binary => {
                let mut left = self.left.as_mut().unwrap().evaluate_expr();
                let mut right = self.right.as_mut().unwrap().evaluate_expr();

                match self.operator.token_type {
                    // Arithmetic/Concatenation operators
                    token::TokenType::Minus => {
                        if !Expr::is_valid_arithmetic(left.literal_type, right.literal_type) {
                            self.print_custom_error(&format!("Subtraction cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                            panic!();
                        }
                        if left.literal_type == LiteralType::Number {
                            // Normal subtraction
                            let r = Expr::string_to_float(&left) - Expr::string_to_float(&right);
                            return Literal::new_value(left.literal_type, r.to_string());
                        }
                        else {
                            // Array subtraction
                            let (left_array, right_array) = (left.array_values, right.array_values);
                            if left_array.len() != right_array.len() {
                                self.print_custom_error(&format!("Cannot subtract array of different sizes, {} and {},", left_array.len(), right_array.len()));
                            }
                            
                            if left_array.len() == 0 {
                                return Literal::new_array(Vec::<Literal>::new());
                            }
                            else{
                                let left_array_type = left_array.get(0).unwrap().literal_type;
                                let right_array_type = right_array.get(0).unwrap().literal_type;
                                if left_array_type != right_array_type {
                                    self.print_custom_error(&format!("Arrays are not of the same type. Left array is of type {:?} but right array is of type {:?}", left_array_type, right_array_type));
                                }
                                if left_array_type == LiteralType::Number && right_array_type == LiteralType::Number {
                                    // Subtract using rayon's iteration
                                    let result_array = left_array.par_iter()
                                                        .zip(right_array.par_iter())
                                                        .map(
                                                            |(a, b)|
                                                            Literal::number((Expr::string_to_float(&a) - Expr::string_to_float(&b)).to_string())
                                                        )
                                                        .collect();
                                    return Literal::new_array(result_array);
                                }
                                else {
                                    self.print_custom_error(&format!("Array subtraction cannot be applied to {:?} and {:?}", left_array_type, right_array_type));
                                    panic!();
                                }
                            }
                        }
                    },
                    token::TokenType::Slash => {
                        if !Expr::is_valid_arithmetic(left.literal_type, right.literal_type) {
                            self.print_custom_error(&format!("Division cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                            panic!();
                        }
                        if left.literal_type == LiteralType::Number {
                            // Normal division
                            let r = match Expr::divide(&left, &right) {
                                Ok(v) => v,
                                Err(()) => {
                                    self.print_custom_error("Division by zero occurs");
                                    panic!();
                                }
                            };
                            return Literal::new_value(left.literal_type, r.to_string());
                        }
                        else {
                            // Array division
                            let (left_array, right_array) = (&mut left.array_values, &mut right.array_values);
                            if left_array.len() != right_array.len() {
                                self.print_custom_error(&format!("Cannot divide array of different sizes, {} and {},", left_array.len(), right_array.len()));
                            }
                            
                            if left_array.len() == 0 {
                                return Literal::new_array(Vec::<Literal>::new());
                            }
                            else{
                                let left_array_type = left_array.get(0).unwrap().literal_type;
                                let right_array_type = right_array.get(0).unwrap().literal_type;
                                if left_array_type != right_array_type {
                                    self.print_custom_error(&format!("Arrays are not of the same type. Left array is of type {:?} but right array is of type {:?}", left_array_type, right_array_type));
                                }
                                if left_array_type == LiteralType::Number && right_array_type == LiteralType::Number {
                                    // Divide using rayon's iteration
                                    let result_array = match left_array.par_iter()
                                                        .zip(right_array.par_iter())
                                                        .map(
                                                            |(a, b)| -> Result<Literal, ()> {
                                                                match Expr::divide(&a, &b) {
                                                                    Ok(v) => Ok(Literal::number(v.to_string())),
                                                                    Err(()) => Err(())
                                                                }
                                                                
                                                            }
                                                        )
                                                        .collect() 
                                                        {
                                                            Ok(arr) => arr,
                                                            Err(_) => {
                                                                self.print_custom_error("Division by zero in one of the array elements occurs");
                                                                panic!();
                                                            }
                                                        };
                                    return Literal::new_array(result_array);
                                }
                                else {
                                    self.print_custom_error(&format!("Array division cannot be applied to {:?} and {:?}", left_array_type, right_array_type));
                                    panic!();
                                }
                            }
                        }
                    },
                    token::TokenType::Star => {
                        if !Expr::is_valid_arithmetic(left.literal_type, right.literal_type) {
                            self.print_custom_error(&format!("Multiplication cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                            panic!();
                        }
                        if left.literal_type == LiteralType::Number {
                            // Normal multiplication
                            let r = Expr::string_to_float(&left) * Expr::string_to_float(&right);
                            return Literal::new_value(left.literal_type, r.to_string());
                        }
                        else {
                            // Array multiplication
                            let (left_array, right_array) = (left.array_values, right.array_values);
                            if left_array.len() != right_array.len() {
                                self.print_custom_error(&format!("Cannot multiply array of different sizes, {} and {},", left_array.len(), right_array.len()));
                            }
                            
                            if left_array.len() == 0 {
                                return Literal::new_array(Vec::<Literal>::new());
                            }
                            else{
                                let left_array_type = left_array.get(0).unwrap().literal_type;
                                let right_array_type = right_array.get(0).unwrap().literal_type;
                                if left_array_type != right_array_type {
                                    self.print_custom_error(&format!("Arrays are not of the same type. Left array is of type {:?} but right array is of type {:?}", left_array_type, right_array_type));
                                }
                                if left_array_type == LiteralType::Number && right_array_type == LiteralType::Number {
                                    // Subtract using rayon's iteration
                                    let result_array = left_array.par_iter()
                                                        .zip(right_array.par_iter())
                                                        .map(
                                                            |(a, b)|
                                                            Literal::number((Expr::string_to_float(&a) * Expr::string_to_float(&b)).to_string())
                                                        )
                                                        .collect();
                                    return Literal::new_array(result_array);
                                }
                                else {
                                    self.print_custom_error(&format!("Array multiplication cannot be applied to {:?} and {:?}", left_array_type, right_array_type));
                                    panic!();
                                }
                            }
                        }
                    },
                    token::TokenType::Plus => {
                        // Applies to number, string, mixed, and their array counterparts
                        ///////////////////////////////////////////////////////////////////////////////////
                        // Should I implement array concatenation? Maybe not here. Try the native functions
                        ///////////////////////////////////////////////////////////////////////////////////

                        let mixed_concat = match Expr::add_or_concat(left.literal_type, right.literal_type) {
                            Ok(v) => v,
                            Err(_) => {
                                self.print_custom_error(&format!("Addition cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                                panic!();
                            }
                        };
                        match left.literal_type {
                            LiteralType::Number => {
                                return Expr::add(&left, &right, mixed_concat);
                            },
                            LiteralType::String => {
                                return Expr::add(&left, &right, true);
                            },
                            LiteralType::Array => {
                                // Array addition
                                let (left_array, right_array) = (left.array_values, right.array_values);
                                if left_array.len() != right_array.len() {
                                    self.print_custom_error(&format!("Cannot add array of different sizes, {} and {},", left_array.len(), right_array.len()));
                                }
                                
                                if left_array.len() == 0 {
                                    return Literal::new_array(Vec::<Literal>::new());
                                }
                                else{
                                    let left_array_type = left_array.get(0).unwrap().literal_type;
                                    let right_array_type = right_array.get(0).unwrap().literal_type;
                                    let mixed_concat = match Expr::add_or_concat(left_array_type, right_array_type) {
                                        Ok(v) => v,
                                        Err(_) => {
                                            self.print_custom_error(&format!("Arrays are not of the same type. Left array is of type {:?} but right array is of type {:?}", left_array_type, right_array_type));
                                            panic!();
                                        }
                                    };
                                    if left_array_type == LiteralType::Number {
                                        //
                                        // Addition using rayon's iteration
                                        let result_array = left_array.par_iter()
                                                            .zip(right_array.par_iter())
                                                            .map(
                                                                |(a, b)|
                                                                Expr::add(&a, &b, mixed_concat)
                                                            )
                                                            .collect();
                                        return Literal::new_array(result_array);
                                    }
                                    else if left_array_type == LiteralType::String {
                                        // String concatenation using rayon's iteration
                                        let result_array = left_array.par_iter()
                                                            .zip(right_array.par_iter())
                                                            .map(
                                                                |(a, b)|
                                                                Expr::add(&a, &b, true)
                                                            )
                                                            .collect();
                                        return Literal::new_array(result_array);
                                    }
                                    else {
                                        self.print_custom_error(&format!("Array addition cannot be applied to {:?} and {:?}", left_array_type, right_array_type));
                                        panic!();
                                    }
                                }
                            },
                            _ => {
                                self.print_custom_error(&format!("Addition cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                                panic!();
                            }
                        };
                    },

                    // Equality operators
                    token::TokenType::Greater => {
                        if !Expr::is_valid_arithmetic(left.literal_type, right.literal_type) {
                            self.print_custom_error(&format!("'Greater than' (>) cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                            panic!();
                        }
                        let result : bool = Expr::string_to_float(&left) > Expr::string_to_float(&right);
                        return Literal::bool(result);
                    },
                    token::TokenType::GreaterEqual => {
                        if !Expr::is_valid_arithmetic(left.literal_type, right.literal_type) {
                            self.print_custom_error(&format!("'Greater-or-equal than' (>=) cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                            panic!();
                        }
                        let result : bool = Expr::string_to_float(&left) >= Expr::string_to_float(&right);
                        return Literal::bool(result);
                    },
                    token::TokenType::Less => {
                        if !Expr::is_valid_arithmetic(left.literal_type, right.literal_type) {
                            self.print_custom_error(&format!("'Lesser than' (<) cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                            panic!();
                        }
                        let result : bool = Expr::string_to_float(&left) < Expr::string_to_float(&right);
                        return Literal::bool(result);
                    },
                    token::TokenType::LessEqual => {
                        if !Expr::is_valid_arithmetic(left.literal_type, right.literal_type) {
                            self.print_custom_error(&format!("'Lesser-or-equal than' (<=) cannot be applied to {:?} and {:?}", left.literal_type, right.literal_type));
                            panic!();
                        }
                        let result : bool = Expr::string_to_float(&left) <= Expr::string_to_float(&right);
                        return Literal::bool(result);
                    },
                    token::TokenType::BangEqual => {
                        let result = !self.is_equal("'Not equals' (!=)", left.literal_type, right.literal_type, &left.value, &right.value);
                        return Literal::bool(result);
                    },
                    token::TokenType::EqualEqual => {
                        // Compare, numbers, strings, bools, null, classes
                        let result = self.is_equal("'Equals' (==)", left.literal_type, right.literal_type, &left.value, &right.value);
                        return Literal::bool(result);
                    },
                    _ => {
                        self.print_custom_error(&format!("{:?} is not a binary operation.", self.operator.token_type));
                        panic!();
                    }
                };
            },
            ExprType::Logical => {
                // (or, and)
                let left_literal = self.left.as_mut().unwrap().evaluate_expr();
                let right_literal = self.right.as_mut().unwrap().evaluate_expr();
                if !Expr::is_truthy(&left_literal) || !Expr::is_truthy(&left_literal) {
                    self.print_custom_error(&format!("'Logical' {:?} cannot be applied to {:?} and {:?}", self.operator.token_type, left_literal.literal_type, right_literal.literal_type));
                }
                match self.operator.token_type {
                    token::TokenType::Or => {
                        if self.string_to_bool(&left_literal) {
                            return left_literal;
                        }
                    },
                    token::TokenType::And => {
                        if !self.string_to_bool(&left_literal) {
                            return left_literal;
                        }
                    },
                    _ => {
                        self.print_custom_error(&format!("{:?} is not a logical operation.", self.operator.token_type));
                    }
                }
                return right_literal;
            },
            ExprType::Unary => {
                let literal = self.right.as_mut().unwrap().evaluate_expr();
                match self.operator.token_type {
                    token::TokenType::Minus => {
                        if literal.literal_type != LiteralType::Number {
                            self.print_custom_error(&format!("'Sign reversal' (-) cannot be applied to {:?}", literal.literal_type));
                        }
                        /*
                        if right_string.chars().nth(0).unwrap() == '-' {
                            right_string.retain(|c| !r#"-"#.contains(c));
                        }
                        else{
                            right_string = "-".to_string() + &right_string;
                        }
                        */
                        let value = - Expr::string_to_float(&literal);
                        return Literal::new_value(literal.literal_type, value.to_string());
                    },
                    token::TokenType::Bang => {
                        //let right_string = literal.value.to_owned();
                        if !Expr::is_truthy(&literal) {
                            self.print_custom_error(&format!("'Negation' (!) cannot be applied to {:?}", literal.literal_type));
                        }
                        let result = match literal.value.as_str() {
                            "true" => {
                                false
                            },
                            "false" => {
                                true
                            }
                            "null" => {
                                true
                            }
                            _ => {
                                self.print_custom_error("'Boolean reversal' (!) only handles 'true', 'false', 'null' values.");
                                panic!();
                            }
                        };
                        return Literal::bool(result);
                    },
                    _ => {
                        self.print_custom_error(&format!("{:?} is not a unary operation.", self.operator.token_type));
                        panic!();
                    }
                };
            },
            ExprType::Grouping => {
                return self.right.as_mut().unwrap().evaluate_expr();
            },

            ExprType::Literal => {
                return self.literal.clone();
            },

            ExprType::Variable => {
                return ENV.lock().unwrap().get_variable(&self.operator);
            },

            ExprType::Assign => {
                let literal_value = self.right.as_mut().unwrap().evaluate_expr();
                ENV.lock().unwrap().assign_variable(&self.operator, literal_value.clone());
                return Literal::none();
            },

            // For assigning specific value to array
            ExprType::ArrayAssign => {
                // self.operator refers to the variable token
                let mut array_reference = ENV.lock().unwrap().get_variable(&self.operator);

                if array_reference.literal_type == LiteralType::Array {
                    let index_literal = self.left.as_mut().unwrap().evaluate_expr();

                    if index_literal.literal_type != LiteralType::Number {
                        self.print_custom_error(&format!("{:?} is not a valid array index type. Only positive integers are allowed", index_literal.literal_type));
                    }
                    let index_float = Expr::string_to_float(&index_literal);
                    if index_float.fract() != 0.0 {
                        self.print_custom_error(&format!("{} is a float and is not a valid array index. Only positive integers are allowed", index_float));
                    }
                    let index_integer = index_float as i32;
                    if index_integer < 0 {
                        self.print_custom_error(&format!("{} is negative and is not a valid array index. Only positive integers are allowed", index_float));
                    }

                    // Set new value
                    let literal_value = self.right.as_mut().unwrap().evaluate_expr();

                    if array_reference.array_values.len() == 0 {
                        if index_integer == 0 {
                            // Push to empty array
                            array_reference.array_values.push(literal_value);
                        }
                        else {
                            self.print_custom_error(&format!("Attempt to modify empty array with index {}. Can only modify with index 0", index_float));
                        }
                    }
                    else {
                        match array_reference.array_values.get(index_integer as usize) {
                            Some(_) => {},
                            None => {
                                self.print_custom_error(&format!("Attempt to modify non-existent index in array with {}", index_float));
                            }
                        };
                        let original_type = array_reference.array_values.get(0).unwrap();
                        if original_type.literal_type != literal_value.literal_type {
                            self.print_custom_error(&format!("Array values are not of the same type. Index 0 is of type {:?} but new value is of type {:?}", original_type.literal_type, literal_value.literal_type));
                        }
                        let _= std::mem::replace(&mut array_reference.array_values[index_integer as usize], literal_value);
                    }
                    ENV.lock().unwrap().assign_variable(&self.operator, array_reference);
                }
                else {
                    self.print_custom_error(&format!("{:?} is not an array and cannot be indexed", array_reference.literal_type));
                }

                return Literal::none();
            },

            // For Array creation
            ExprType::ArrayCreation => {
                if self.arguments.len() == 0 {
                    //self.print_custom_error(&format!("Cannot declare empty array"));
                }
                let mut values = Vec::<Literal>::new();
                let mut value_type = LiteralType::None; // Keep track of array type
                let mut index = 0 ;
                let mut error = false;
                let mut error_literal_type = LiteralType::None;
                // Avoid cloning the arguments/values, because they can be large
                for value_expr in &mut self.arguments {
                    let value = value_expr.evaluate_expr();
                    if index == 0 {
                        value_type = value.literal_type;
                    }
                    else if value_type != value.literal_type {
                        error = true;
                        error_literal_type = value.literal_type;
                        break;
                    }
                    values.push(value);
                    index += 1;
                }
                if error {
                    self.print_custom_error(&format!("Array values are not of the same type. Index 0 is of type {:?} but index {} is of type {:?}", value_type, index, error_literal_type));
                }

                return Literal::new_array(values);
            },
            // For Array access
            ExprType::ArrayAccess => {
                let array_reference = self.left.as_mut().unwrap().evaluate_expr();
                if array_reference.literal_type == LiteralType::Array {
                    let index_literal = self.right.as_mut().unwrap().evaluate_expr();
                    if index_literal.literal_type != LiteralType::Number {
                        self.print_custom_error(&format!("{:?} is not a valid array index type. Only positive integers are allowed", index_literal.literal_type));
                    }
                    let index_float = Expr::string_to_float(&index_literal);
                    if index_float.fract() != 0.0 {
                        self.print_custom_error(&format!("{} is a float and is not a valid array index. Only positive integers are allowed", index_float));
                    }
                    let index_integer = index_float as i32;
                    if index_integer < 0 {
                        self.print_custom_error(&format!("{} is negative and is not a valid array index. Only positive integers are allowed", index_float));
                    }
                    match array_reference.array_values.get(index_integer as usize) {
                        Some(result) => result.clone(),
                        None => {
                            self.print_custom_error(&format!("Attempt to access non-existent index in array with {}", index_float));
                            panic!();
                        }
                    }
                }
                else {
                    self.print_custom_error(&format!("{:?} is not an array and cannot be indexed", array_reference.literal_type));
                    panic!();
                }
            }

            // For function calling/invocation, not declaration 
            ExprType::Call => {
                let callee = self.right.as_mut().unwrap().evaluate_expr();
                let mut arguments = Vec::<Literal>::new();
                for arg in &mut self.arguments {
                    arguments.push(arg.evaluate_expr());
                }
                if callee.literal_type != LiteralType::Function {
                    self.print_custom_error(&format!("{:?} is not a function that can be called", callee.literal_type));
                }
                let mut function = callee.function.unwrap();
                if function.arg_length() != arguments.len() {
                    self.print_custom_error(&format!("Function expects {} arguments, but received {} arguments instead", function.arg_length(), arguments.len()));
                }
                match function.call(arguments) {
                    Some(literal) => {
                        literal
                    },
                    None => {
                        self.print_custom_error(&format!("Cannot invoke Function of type 'None'"));
                        Literal::none()
                    }
                }
            },
            ExprType::None => {
                return Literal::none();
            }

            _ => {
                self.print_custom_error(&format!("evaluateExpr() does not account for {:?}", self.expr_type));
                panic!();
            }
        }
    }
    fn print_error(&self, error: ari_errors::ErrorType){
        self.operator.print_error(error);
    }
    fn print_custom_error(&self, message: &str){
        self.operator.print_custom_error(message);
    }
}