use crate::token;
use crate::ast;
use crate::ast::Expr;
use crate::environment::Environment;
use crate::environment::ENV;
//use rayon::prelude::*; // For array operations/fast parallelism

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)] // For equality comparisons
pub enum FunctionType {
    UserDefined, // Uses 'branch' which is defined by user
    Native, // Uses 'closure' which is pre-defined by Rust code

    None, // Placeholder
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)] // For equality comparisons
pub enum NativeType {
    // Number operations
    Power,
    Log,
    Modulo,
    Absolute,
    Floor,
    Ceiling,
    Max,
    Min,
    
    // String/Number conversions
    ToString,
    ToNumber,

    // String operations
    Split,
    ToLowercase,
    ToUpperCase,

    // Array operations
    Length, // Also works for string
    Insert, // Also works for string
    Remove, // Also works for string

    Map,
    Filter,
    Reduce,

    Range,
    Linspace,
    Repeat,

    // Random generation
    RandomChoose,
    RandomNormal,

    // File operations
    ReadFile,
    WriteFile,

    // Web
    ServeStaticFolder,
    WebGet,
    WebPost,

    None, // Placeholder
}

pub struct Function {
    function_type: FunctionType,
    arguments: Vec<token::Token>,
    user_defined: Option<Box<ast::Statement>>,
    native_type: NativeType,
    pub closure_env: Option<Environment>,
    pub variable_token: token::Token, // For updating the closure in the environment
}

impl Clone for Function { // Enables Function to be copied
    fn clone(&self) -> Function {
        Function {
            function_type: self.function_type,
            arguments: self.arguments.clone(),
            user_defined: self.user_defined.clone(),
            native_type: self.native_type,
            closure_env: self.closure_env.clone(),
            variable_token: self.variable_token.clone(),
        }
    }
}

impl Function {
    pub fn new(function_type: FunctionType, arguments: Vec<token::Token>, user_defined: Option<Box<ast::Statement>>, native_type: NativeType,
                closure_env: Option<Environment>, variable_token: token::Token) -> Function {
        Function {
            function_type,
            arguments,
            user_defined,
            native_type,
            closure_env,
            variable_token,
        }
    }
    pub fn new_user(arguments: Vec<token::Token>, user_defined: Option<Box<ast::Statement>>, closure_env: Environment, variable_token: token::Token) -> Function {
        Function::new(FunctionType::UserDefined, arguments, user_defined, NativeType::None, Some(closure_env), variable_token)
    }
    pub fn new_native(native_type: NativeType) -> Function {
        let number_of_args = Function::number_of_args(native_type);
        Function::new(FunctionType::Native, Vec::<token::Token>::with_capacity(number_of_args), None, native_type, None, token::Token::none())
    }
    pub fn none() -> Function {
        Function::new(FunctionType::None, Vec::<token::Token>::new(), None, NativeType::None, None, token::Token::none())
    }

    pub fn call(&self, arguments: Vec<ast::Literal>, tok: &token::Token) -> Option<ast::Literal> {
        
        let result = match self.function_type {
            FunctionType::UserDefined => {
                //println!("Invoke user! {}", self.arguments.len());
                ENV.lock().unwrap().add_env(self.closure_env.as_ref().unwrap().clone());
                
                ENV.lock().unwrap().create_env();
                let r = Some(self.call_user(arguments));
                ENV.lock().unwrap().destroy_env();

                let cloned = Some(ENV.lock().unwrap().get_env().clone());
                ENV.lock().unwrap().destroy_env();

                let mut updated_function = self.clone();
                updated_function.closure_env = cloned;
                ENV.lock().unwrap().assign_variable(&self.variable_token, ast::Literal::new_function(updated_function));
                r
            },
            FunctionType::Native => {
                //println!("Invoke native! {}", self.arguments.len());
                ENV.lock().unwrap().create_env();
                let r = Some(self.call_native(arguments, tok));
                ENV.lock().unwrap().destroy_env();
                r
            },
            _ => {
                None
            }
        };
        return result;
    }

    pub fn call_user(&self, arguments: Vec<ast::Literal>) -> ast::Literal {
        for i in 0..arguments.len() {
            // Insert arg name: arg value into new scope
            ENV.lock().unwrap().get_env().define(self.arguments.get(i).unwrap().lexeme.to_string(), arguments.get(i).unwrap().clone());
        }
        return self.user_defined.as_ref().unwrap().evaluate_statement()
    }

    pub fn call_native(&self, arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
        match self.native_type {
            NativeType::Power => {
                power(arguments, tok)
            },
            NativeType::Log => {
                log(arguments, tok)
            },
            NativeType::Modulo => {
                modulo(arguments, tok)
            },
            NativeType::Absolute => {
                absolute(arguments, tok)
            },
            NativeType::Floor => {
                floor(arguments, tok)
            },
            NativeType::Ceiling => {
                ceiling(arguments, tok)
            },
            NativeType::Max => {
                max(arguments, tok)
            },
            NativeType::Min => {
                min(arguments, tok)
            },
            NativeType::ToString => {
                to_string(arguments, tok)
            },
            NativeType::ToNumber => {
                to_number(arguments, tok)
            },
            NativeType::Split => {
                split(arguments, tok)
            },
            NativeType::ToLowercase => {
                to_lowercase(arguments, tok)
            },
            NativeType::ToUpperCase => {
                to_uppercase(arguments, tok)
            },
            NativeType::Length => {
                length(arguments, tok)
            },
            NativeType::Insert => {
                insert(arguments, tok)
            },
            NativeType::Remove => {
                remove(arguments, tok)
            },
            NativeType::Map => {
                map(arguments, tok)
            },
            NativeType::Filter => {
                filter(arguments, tok)
            },
            NativeType::Reduce => {
                reduce(arguments, tok)
            },
            NativeType::Range => {
                range(arguments, tok)
            },
            NativeType::Linspace => {
                linspace(arguments, tok)
            },
            NativeType::Repeat => {
                repeat(arguments, tok)
            },
            NativeType::RandomChoose => {
                random_choose(arguments, tok)
            },
            NativeType::RandomNormal => {
                random_normal(arguments, tok)
            },
            NativeType::ReadFile => {
                read_file(arguments, tok)
            },
            NativeType::WriteFile => {
                write_file(arguments, tok)
            },
            NativeType::ServeStaticFolder => {
                serve_static_folder(arguments, tok)
            },
            NativeType::WebGet => {
                web_get(arguments, tok)
            },
            NativeType::WebPost => {
                web_post(arguments, tok)
            },
            _ => {
                panic!("call_native() has not accounted for {:?}", self.native_type);
            }
        }
    }
    pub fn arg_length(&self) -> usize {
        if self.function_type == FunctionType::UserDefined {
            self.arguments.len()
        }
        else {
            Function::number_of_args(self.native_type)
        }
    }
    pub fn number_of_args(native_type: NativeType) -> usize {
        match native_type {
            // Number operations
            NativeType::Power =>    2,
            NativeType::Log =>      2,
            NativeType::Modulo =>   2,
            NativeType::Absolute => 1,
            NativeType::Floor =>    1,
            NativeType::Ceiling =>  1,
            NativeType::Max =>      2,
            NativeType::Min =>      2,
            //String/Number conversions
            NativeType::ToString => 1,
            NativeType::ToNumber => 1,
            //String operations
            NativeType::Split =>        2,
            NativeType::ToLowercase =>  1,
            NativeType::ToUpperCase =>  1,
            //Array operations
            NativeType::Length =>       1,
            NativeType::Insert =>       3,
            NativeType::Remove =>       2,

            NativeType::Map =>          2,
            NativeType::Filter =>       2,
            NativeType::Reduce =>       3,

            NativeType::Range =>        3,
            NativeType::Linspace =>     3,
            NativeType::Repeat =>       2,

            // Random generation
            NativeType::RandomChoose => 2,
            NativeType::RandomNormal => 3,

            // File operations
            NativeType::ReadFile =>     1,
            NativeType::WriteFile =>    2,
            
             // Web
             NativeType::ServeStaticFolder =>   3,
             NativeType::WebGet =>              1,
             NativeType::WebPost =>             2,

            _ => {
                panic!("new_native() has not accounted for {:?}", native_type);
            }
        }
    }
}

////////////////////
/// Native Functions
////////////////////
// Number operations
fn power(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let base = arguments.get(0).unwrap();
    let power = arguments.get(1).unwrap();
    if base.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("power() expects 1st argument (base) of type Number, but received {:?} instead", base.literal_type));
    }
    else if power.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("power() expects 2nd argument (power) of type Number, but received {:?} instead", power.literal_type));
    }
    else {
        return ast::Literal::number(Expr::string_to_float(&base).powf(Expr::string_to_float(&power)).to_string());
    }
    ast::Literal::none()
}
fn log(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let base = arguments.get(0).unwrap();
    let value = arguments.get(1).unwrap();
    if base.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("log() expects 1st argument (base) of type Number, but received {:?} instead", base.literal_type));
    }
    else if value.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("log() expects 2nd argument (value) of type Number, but received {:?} instead", value.literal_type));
    }
    else {
        let result = Expr::string_to_float(&value).log(Expr::string_to_float(&base));
        if result.is_infinite() {
            tok.print_custom_error(&format!("log() expects 2nd argument (value) to be non-zero"));
        }
        else if result.is_nan() {
            tok.print_custom_error(&format!("log() expects 1st argument (base) to be non-zero"));
        }
        return ast::Literal::number(result.to_string());
    }
    ast::Literal::none()
}
fn modulo(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    let modulee = arguments.get(1).unwrap();
    if value.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("modulo() expects 1st argument (value) of type Number, but received {:?} instead", value.literal_type));
    }
    else if modulee.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("modulo() expects 2nd argument (modulee) of type Number, but received {:?} instead", modulee.literal_type));
    }
    else {
        let value_float = Expr::string_to_float(&value);
        if value_float.fract() != 0.0 {
            tok.print_custom_error(&format!("modulo() expects 1st argument (value) to be an integer, but received {} instead", value_float));
        }
        let modulee_float = Expr::string_to_float(&modulee);
        if modulee_float.fract() != 0.0 {
            tok.print_custom_error(&format!("modulo() expects 2nd argument (modulee) to be an integer, but received {} instead", modulee_float));
        }
        if (1.0 / modulee_float).is_infinite() {
            tok.print_custom_error(&format!("modulo() expects 2nd argument (modulee) to be non-zero"));
        }
        let value_integer = value_float as i32;
        let modulee_integer = modulee_float as i32;
        let result = value_integer % modulee_integer;
        return ast::Literal::number(result.to_string());
    }
    ast::Literal::none()
}
fn absolute(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("absolute() expects one argument of type Number, but received {:?} instead", value.literal_type));
    }
    else {
        return ast::Literal::number(Expr::string_to_float(&value).abs().to_string());
    }
    ast::Literal::none()
}
fn floor(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("floor() expects one argument of type Number, but received {:?} instead", value.literal_type));
    }
    else {
        return ast::Literal::number(Expr::string_to_float(&value).floor().to_string());
    }
    ast::Literal::none()
}
fn ceiling(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("ceiling() expects one argument of type Number, but received {:?} instead", value.literal_type));
    }
    else {
        return ast::Literal::number(Expr::string_to_float(&value).ceil().to_string());
    }
    ast::Literal::none()
}
fn max(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let left = arguments.get(0).unwrap();
    let right = arguments.get(1).unwrap();
    if left.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("max() expects 1st argument (left) of type Number, but received {:?} instead", left.literal_type));
    }
    else if right.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("max() expects 2nd argument (right) of type Number, but received {:?} instead", right.literal_type));
    }
    else {
        let left_float = Expr::string_to_float(&left);
        let right_float = Expr::string_to_float(&right);
        let result = if left_float > right_float {
            left_float
        }
        else {
            right_float
        };
        return ast::Literal::number(result.to_string());
    }
    ast::Literal::none()
}
fn min(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let left = arguments.get(0).unwrap();
    let right = arguments.get(1).unwrap();
    if left.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("min() expects 1st argument (left) of type Number, but received {:?} instead", left.literal_type));
    }
    else if right.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("min() expects 2nd argument (right) of type Number, but received {:?} instead", right.literal_type));
    }
    else {
        let left_float = Expr::string_to_float(&left);
        let right_float = Expr::string_to_float(&right);
        let result = if left_float < right_float {
            left_float
        }
        else {
            right_float
        };
        return ast::Literal::number(result.to_string());
    }
    ast::Literal::none()
}
fn to_string(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("to_string() expects one argument of type Number, but received {:?} instead", value.literal_type));
    }
    else {
        return ast::Literal::string(value.value.clone());
    }
    ast::Literal::none()
}
fn to_number(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("to_number() expects one argument of type String, but received {:?} instead", value.literal_type));
    }
    else {
        let result = match value.value.parse::<f32>() {
            Ok(v) => {
                v
            },
            Err(_) => {
                tok.print_custom_error(&format!("to_number() failed to extract a Number from {}", value.value));
                panic!();
            }
        };
        return ast::Literal::number(result.to_string());
    }
    ast::Literal::none()
}

// String operations
fn split(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let source = arguments.get(0).unwrap();
    let delimiter = arguments.get(1).unwrap();
    if source.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("split() expects 1st argument (source) of type String, but received {:?} instead", source.literal_type));
    }
    else if delimiter.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("split() expects 2nd argument (delimiter) of type String, but received {:?} instead", delimiter.literal_type));
    }
    else {
        let result_array = source.value.split(&delimiter.value).map(|value| ast::Literal::string(value.to_string().clone())).collect();
        return ast::Literal::new_array(result_array);
    }
    ast::Literal::none()
}
fn to_lowercase(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("to_lowercase() expects one argument of type String, but received {:?} instead", value.literal_type));
    }
    else {
        return ast::Literal::string(value.value.to_lowercase().clone());
    }
    ast::Literal::none()
}
fn to_uppercase(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("to_uppercase() expects one argument of type String, but received {:?} instead", value.literal_type));
    }
    else {
        return ast::Literal::string(value.value.to_uppercase().clone());
    }
    ast::Literal::none()
}

// Array operations
fn length(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let value = arguments.get(0).unwrap();
    if value.literal_type != ast::LiteralType::Array && value.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("length() expects one argument of type Array or String, but received {:?} instead", value.literal_type));
    }
    else {
        if value.literal_type == ast::LiteralType::Array {
            // Length of array
            return ast::Literal::number(value.array_values.len().to_string());
        }
        else {
            // Length of string
            return ast::Literal::number(value.value.len().to_string());
        }
    }
    ast::Literal::none()
}
fn insert(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let source = arguments.get(0).unwrap();
    let index = arguments.get(1).unwrap();
    let new_value = arguments.get(2).unwrap();
    
    if source.literal_type != ast::LiteralType::Array && source.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("insert() expects 1st argument (source) of type Array or String, but received {:?} instead", source.literal_type));
    }
    else if index.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("insert() expects 2nd argument (index) of type Number, but received {:?} instead", index.literal_type));
    }
    else {
        // Do some index checks
        if index.literal_type != ast::LiteralType::Number {
            tok.print_custom_error(&format!("{:?} is not a valid array index type for insert(). Only positive integers are allowed", index.literal_type));
        }
        let index_float = Expr::string_to_float(&index);
        if index_float.fract() != 0.0 {
            tok.print_custom_error(&format!("{} is a float and is not a valid array index for insert(). Only positive integers are allowed", index_float));
        }
        let index_integer = index_float as i32;
        if index_integer < 0 {
            tok.print_custom_error(&format!("{} is negative and is not a valid array index for insert(). Only positive integers are allowed", index_float));
        }
        let index_integer = index_integer as usize;
        if source.literal_type == ast::LiteralType::Array {
            // Array insert
            if new_value.literal_type != ast::LiteralType::Array {
                tok.print_custom_error(&format!("insert() expects new value of type Array, but received {:?} instead", new_value.literal_type));
            }
            let mut source_array = source.array_values.clone();
            if source_array.len() == 0 {
                if index_integer == 0 {
                    source_array.push(new_value.clone());
                }
                tok.print_custom_error(&format!("insert() cannot insert at {} because the array is empty.", index_integer));
            }
            if index_integer > source_array.len() {
                tok.print_custom_error(&format!("insert() cannot insert at {} because it is beyond the array's bounds.", index_integer));
            }
            let original_type = source_array.get(0).unwrap().literal_type;
            let new_type = new_value.array_values.get(0).unwrap().literal_type;
            if new_type != original_type {
                tok.print_custom_error(&format!("insert() expects 3rd argument (value) of type {:?}, but received {:?} instead", original_type, new_type));
            }
            source_array.splice(index_integer..index_integer, new_value.array_values.iter().cloned());
            return ast::Literal::new_array(source_array);
        }
        else {
            // String insert
            let mut source_string = source.value.clone();
            if index_integer > source_string.len() {
                tok.print_custom_error(&format!("insert() cannot insert at {} because it is beyond the string's bounds.", index_integer));
            }
            if new_value.literal_type != ast::LiteralType::String {
                tok.print_custom_error(&format!("insert() expects 3rd argument (value) of type String, but received {:?} instead", new_value.literal_type));
            }
            source_string.insert_str(index_integer, &new_value.value);
            return ast::Literal::string(source_string);
        }
    }
    ast::Literal::none()
}
fn remove(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let source = arguments.get(0).unwrap();
    let index = arguments.get(1).unwrap();
    
    if source.literal_type != ast::LiteralType::Array && source.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("remove() expects 1st argument (source) of type Array or String, but received {:?} instead", source.literal_type));
    }
    else if index.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("remove() expects 2nd argument (index) of type Number, but received {:?} instead", index.literal_type));
    }
    else {
        // Do some index checks
        if index.literal_type != ast::LiteralType::Number {
            tok.print_custom_error(&format!("{:?} is not a valid array index type for remove(). Only positive integers are allowed", index.literal_type));
        }
        let index_float = Expr::string_to_float(&index);
        if index_float.fract() != 0.0 {
            tok.print_custom_error(&format!("{} is a float and is not a valid array index for remove(). Only positive integers are allowed", index_float));
        }
        let index_integer = index_float as i32;
        if index_integer < 0 {
            tok.print_custom_error(&format!("{} is negative and is not a valid array index for remove(). Only positive integers are allowed", index_float));
        }
        let index_integer = index_integer as usize;
        if source.literal_type == ast::LiteralType::Array {
            // Array remove
            let mut source_array = source.array_values.clone();
            if source_array.len() == 0 {
                tok.print_custom_error(&format!("remove() cannot remove at {} because the array is empty.", index_integer));
            }
            if index_integer > source_array.len() {
                tok.print_custom_error(&format!("remove() cannot remove at {} because it is beyond the array's bounds.", index_integer));
            }
            source_array.remove(index_integer);
            return ast::Literal::new_array(source_array);
        }
        else {
            // String remove
            let mut source_string = source.value.clone();
            if index_integer > source_string.len() {
                tok.print_custom_error(&format!("remove() cannot remove at {} because it is beyond the string's bounds.", index_integer));
            }
            source_string.remove(index_integer);
            return ast::Literal::string(source_string);
        }
    }
    ast::Literal::none()
}

fn map(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    let source = arguments.get(0).unwrap();
    let map_function = arguments.get(1).unwrap();
    
    if source.literal_type != ast::LiteralType::Array {
        tok.print_custom_error(&format!("map() expects 1st argument (source) of type Array, but received {:?} instead", source.literal_type));
    }
    else if map_function.literal_type != ast::LiteralType::Function {
        tok.print_custom_error(&format!("map() expects 2nd argument (function) of type Function, but received {:?} instead", map_function.literal_type));
    }
    else {
        let function = map_function.function.as_ref().unwrap();
        if function.arg_length() != 1 {
            tok.print_custom_error(&format!("map() expects a function with 1 argument, but received one with {} arguments instead", function.arg_length()));
        }
        // Array map
        let source_array = &source.array_values;
        let result_array = source_array.iter()
                                        .map(
                                            |a|
                                            {
                                                match function.call(vec![a.clone()], &tok) {
                                                    Some(literal) => {
                                                        literal
                                                    },
                                                    None => {
                                                        tok.print_custom_error(&format!("map() cannot invoke Function of type 'None'"));
                                                        ast::Literal::none()
                                                    }
                                                }

                                            }
                                        )
                                        .collect();
        return ast::Literal::new_array(result_array);
    }
    ast::Literal::none()
}
///////////////////
// Helper function
fn string_to_bool(string : &str) -> bool {
    match string {
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
            panic!("Boolean operation only handles 'true', 'false', 'null' values.");
        }
    }
}
///////////////////////////////
// Continued...Array Operations

fn filter(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns an array of bool Literals
    let source = arguments.get(0).unwrap();
    let filter_function = arguments.get(1).unwrap();
    
    if source.literal_type != ast::LiteralType::Array {
        tok.print_custom_error(&format!("filter() expects 1st argument (source) of type Array, but received {:?} instead", source.literal_type));
    }
    else if filter_function.literal_type != ast::LiteralType::Function {
        tok.print_custom_error(&format!("filter() expects 2nd argument (function) of type Function, but received {:?} instead", filter_function.literal_type));
    }
    else {
        let function = filter_function.function.as_ref().unwrap();
        if function.arg_length() != 1 {
            tok.print_custom_error(&format!("filter() expects a function with 1 argument, but received one with {} arguments instead", function.arg_length()));
        }

        let source_array = &source.array_values;

        // Check if function returns boolean Literal
        if source_array.len() > 0 {
            let return_type = match function.call(vec![source_array.get(0).unwrap().clone()], &tok) {
                Some(literal) => {
                    literal.literal_type
                },
                None => {
                    tok.print_custom_error(&format!("filter() cannot invoke Function of type 'None'"));
                    panic!();
                }
            };
            if return_type != ast::LiteralType::Bool && return_type != ast::LiteralType::Null && return_type != ast::LiteralType::None {
                tok.print_custom_error(&format!("filter() expects 2nd argument (function) to return Bool, but received {:?} instead", return_type));
            }
        }

        // Array filter
        let result_array = source_array.iter().cloned()
                                        .filter(
                                            |a|
                                            {
                                                match function.call(vec![a.clone()], &tok) {
                                                    Some(literal) => {
                                                        if literal.literal_type == ast::LiteralType::None {
                                                            false
                                                        }
                                                        else {
                                                            string_to_bool(&literal.value)
                                                        }
                                                    },
                                                    None => {
                                                        tok.print_custom_error(&format!("filter() cannot invoke Function of type 'None'"));
                                                        panic!();
                                                    }
                                                }
                                            }
                                        )
                                        .collect();
        return ast::Literal::new_array(result_array);
    }
    ast::Literal::none()
}
fn reduce(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns a Literal
    let source = arguments.get(0).unwrap();
    let initial_literal = arguments.get(1).unwrap();
    let filter_function = arguments.get(2).unwrap();
    
    if source.literal_type != ast::LiteralType::Array {
        tok.print_custom_error(&format!("reduce() expects 1st argument (source) of type Array, but received {:?} instead", source.literal_type));
    }
    else if filter_function.literal_type != ast::LiteralType::Function {
        tok.print_custom_error(&format!("reduce() expects 3rd argument (function) of type Function, but received {:?} instead", filter_function.literal_type));
    }
    else {
        let function = filter_function.function.as_ref().unwrap();
        if function.arg_length() != 2 {
            tok.print_custom_error(&format!("reduce() expects a function with 2 arguments, but received one with {} arguments instead", function.arg_length()));
        }

        let source_array = &source.array_values;

        // Check if function returns Literal
        if source_array.len() > 0 {
            let first_element = source_array.get(0).unwrap().clone();
            if initial_literal.literal_type != first_element.literal_type {
                tok.print_custom_error(&format!("2nd argument (initial_value) of reduce() is of type {:?}, but array values are of type {:?}", initial_literal.literal_type, first_element.literal_type));
            }
            let return_type = match function.call(vec![initial_literal.clone(), first_element], &tok) {
                Some(literal) => {
                    literal.literal_type
                },
                None => {
                    tok.print_custom_error(&format!("reduce() cannot invoke Function of type 'None'"));
                    panic!();
                }
            };
            if return_type != initial_literal.literal_type {
                tok.print_custom_error(&format!("reduce() expects 3rd argument (function) to return {:?}, but received {:?} instead", initial_literal.literal_type, return_type));
            }
        }

        // Array reduce
        let result_literal : ast::Literal = source_array.iter()
                                        .fold(
                                            initial_literal.clone(),
                                            |a, b|
                                            {
                                                match function.call(vec![a.clone(), b.clone()], &tok) {
                                                    Some(literal) => {
                                                        literal
                                                    },
                                                    None => {
                                                        tok.print_custom_error(&format!("reduce() cannot invoke Function of type 'None'"));
                                                        panic!();
                                                    }
                                                }
                                            }
                                        );
        return result_literal;
    }
    ast::Literal::none()
}

fn range(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns array of number Literals
    let start = arguments.get(0).unwrap();
    let end = arguments.get(1).unwrap();
    let step = arguments.get(2).unwrap();
    if start.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("range() expects 1st argument (start) of type Number, but received {:?} instead", start.literal_type));
    }
    if end.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("range() expects 2nd argument (end) of type Number, but received {:?} instead", end.literal_type));
    }
    else {
        let mut start_float = Expr::string_to_float(&start);
        let end_float = Expr::string_to_float(&end);
        let step_float = Expr::string_to_float(&step);

        if start_float == end_float {
            return ast::Literal::new_array(vec![start.clone()]);
        }

        // Do some range checks
        let increasing = start_float < end_float;
        if step.literal_type != ast::LiteralType::Number {
            tok.print_custom_error(&format!("{:?} is not a valid step for range()", step.literal_type));
        }
        if (1.0/step_float).is_infinite() {
            tok.print_custom_error(&format!("range() expects a non-zero step from {} to {}", start_float, end_float));
        }
        if increasing && step_float < 0.0 {
            // Increasing, but negative step
            tok.print_custom_error(&format!("range() expects a positive step from {} to {}, but received a {} step instead", start_float, end_float, step_float));
        }
        else if !increasing && step_float > 0.0 {
            // Decreasing, but positive step
            tok.print_custom_error(&format!("range() expects a negative step from {} to {}, but received a {} step instead", start_float, end_float, step_float));
        }
        let mut result_array = Vec::<ast::Literal>::new();
        loop {
            result_array.push(ast::Literal::number(start_float.to_string()));
            if increasing {
                start_float += step_float;
                if start_float > end_float {
                    break;
                }
            }
            else {
                start_float -= step_float;
                if start_float < end_float {
                    break;
                }
            }
        }
        return ast::Literal::new_array(result_array);
    }
    ast::Literal::none()
}
fn linspace(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns array of number Literals
    let start = arguments.get(0).unwrap();
    let end = arguments.get(1).unwrap();
    let num_of_elements = arguments.get(2).unwrap();
    if start.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("linspace() expects 1st argument (start) of type Number, but received {:?} instead", start.literal_type));
    }
    if end.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("linspace() expects 2nd argument (end) of type Number, but received {:?} instead", end.literal_type));
    }
    else {
        // Do some integer checks
        if num_of_elements.literal_type != ast::LiteralType::Number {
            tok.print_custom_error(&format!("{:?} is not a valid value for linspace(). Only positive integers are allowed", num_of_elements.literal_type));
        }
        let num_float = Expr::string_to_float(&num_of_elements);
        if num_float.fract() != 0.0 {
            tok.print_custom_error(&format!("{} is a float and is not a valid value for linspace(). Only positive integers are allowed", num_float));
        }
        let num_integer = num_float as i32;
        if num_integer < 0 {
            tok.print_custom_error(&format!("{} is negative and is not a valid value for linspace(). Only positive integers are allowed", num_integer));
        }
        // Return early if only 0,1, or 2 elements
        let mut num_integer = num_integer as usize;
        {
            match num_integer {
                0 => return ast::Literal::new_array(Vec::<ast::Literal>::new()),
                1 => return ast::Literal::new_array(vec![start.clone()]),
                2 => return ast::Literal::new_array(vec![start.clone(), end.clone()]),
                _ => ()
            };
        }

        let mut start_float = Expr::string_to_float(&start);
        let end_float = Expr::string_to_float(&end);

        if start_float == end_float {
            return ast::Literal::new_array((0..num_integer).map(|_| start.clone()).collect::<Vec<ast::Literal>>());
        }
        let step_float = (end_float - start_float).abs() / ((num_integer - 1) as f32);
        let increasing = start_float < end_float;
        let mut result_array = Vec::<ast::Literal>::new();
        while num_integer > 0 {
            result_array.push(ast::Literal::number(start_float.to_string()));
            if increasing {
                start_float += step_float;
            }
            else {
                start_float -= step_float;
            }
            num_integer -= 1;
        }
        return ast::Literal::new_array(result_array);
    }
    ast::Literal::none()
}
fn repeat(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns array of Literals
    let literal_copy = arguments.get(0).unwrap();
    let num_of_elements = arguments.get(1).unwrap();
    // Do some integer checks
    if num_of_elements.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("{:?} is not a valid repeat value for repeat(). Only positive integers are allowed", num_of_elements.literal_type));
    }
    let num_float = Expr::string_to_float(&num_of_elements);
    if num_float.fract() != 0.0 {
        tok.print_custom_error(&format!("{} is a float and is not a valid repeat value for repeat(). Only positive integers are allowed", num_float));
    }
    let num_integer = num_float as i32;
    if num_integer < 0 {
        tok.print_custom_error(&format!("{} is negative and is not a valid repeat value for repeat(). Only positive integers are allowed", num_integer));
    }
    let num_integer = num_integer as usize;
    let result_array = (0..num_integer).map(|_| literal_copy.clone()).collect::<Vec<ast::Literal>>();
    return ast::Literal::new_array(result_array);
}

// Random generation
use rand_distr::{Distribution, Uniform, Normal};
use rand::thread_rng;

fn random_choose(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Choose a random element of array returns array of number Literals
    let source = arguments.get(0).unwrap();
    let num_of_elements = arguments.get(1).unwrap();
    if source.literal_type != ast::LiteralType::Array {
        tok.print_custom_error(&format!("random_choose() expects 1st argument of type Array, but received {:?} instead", source.literal_type));
    }
    else {
        // Do some integer checks
        if num_of_elements.literal_type != ast::LiteralType::Number {
            tok.print_custom_error(&format!("{:?} is not a valid value for random_choose(). Only positive integers are allowed", num_of_elements.literal_type));
        }
        let num_float = Expr::string_to_float(&num_of_elements);
        if num_float.fract() != 0.0 {
            tok.print_custom_error(&format!("{} is a float and is not a valid value for random_choose(). Only positive integers are allowed", num_float));
        }
        let num_integer = num_float as i32;
        if num_integer < 0 {
            tok.print_custom_error(&format!("{} is negative and is not a valid value for random_choose(). Only positive integers are allowed", num_integer));
        }
        let num_integer = num_integer as usize;
        let source_array = &source.array_values;
        // Generate random array
        let mut rng = thread_rng();
        let uniform = Uniform::from(0..source_array.len());
        let result_array = (0..num_integer).map(|_| source_array[uniform.sample(&mut rng)].clone()).collect::<Vec<ast::Literal>>();
        return ast::Literal::new_array(result_array);
    }
    ast::Literal::none()
}
fn random_normal(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns array of number Literals
    let mean = arguments.get(0).unwrap();
    let std_dev = arguments.get(1).unwrap();
    let num_of_elements = arguments.get(2).unwrap();
    if mean.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("random_normal() expects 1st argument of type Number, but received {:?} instead", mean.literal_type));
    }
    if std_dev.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("random_normal() expects 2nd argument of type Number, but received {:?} instead", std_dev.literal_type));
    }
    else {
        // Do some integer checks
        if num_of_elements.literal_type != ast::LiteralType::Number {
            tok.print_custom_error(&format!("{:?} is not a valid value for random_uniform(). Only positive integers are allowed", num_of_elements.literal_type));
        }
        let num_float = Expr::string_to_float(&num_of_elements);
        if num_float.fract() != 0.0 {
            tok.print_custom_error(&format!("{} is a float and is not a valid value for random_uniform(). Only positive integers are allowed", num_float));
        }
        let num_integer = num_float as i32;
        if num_integer < 0 {
            tok.print_custom_error(&format!("{} is negative and is not a valid value for random_uniform(). Only positive integers are allowed", num_integer));
        }
        let num_integer = num_integer as usize;
        let mean_float = Expr::string_to_float(&mean);
        let std_float = Expr::string_to_float(&std_dev);
    
        // Generate random array
        let mut rng = thread_rng();
        let normal = Normal::new(mean_float, std_float).unwrap();
        let result_array = (0..num_integer).map(|_| ast::Literal::number(normal.sample(&mut rng).to_string())).collect::<Vec<ast::Literal>>();
        return ast::Literal::new_array(result_array);
    }
    ast::Literal::none()
}

// File operations
use std::fs;

fn read_file(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns string Literal if success, null Literal if fail
    let filepath = arguments.get(0).unwrap();
    if filepath.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("read_file() expects one argument of type String, but received {:?} instead", filepath.literal_type));
    }
    let result = match fs::read_to_string(filepath.value.clone()) {
        Ok(content) => ast::Literal::string(content),
        Err(_) => {
            //tok.print_custom_error(&format!("read_file() failed to read file: {}", filepath.value));
            //panic!();
            ast::Literal::null()
        }
    };
    return result;
}

fn write_file(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns 1 if success, 0 if fail
    let filepath = arguments.get(0).unwrap();
    let data = arguments.get(1).unwrap();
    if filepath.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("write_file() expects 1st argument (filepath) of type String, but received {:?} instead", filepath.literal_type));
    }
    if filepath.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("write_file() expects 2nd argument (data) of type String, but received {:?} instead", data.literal_type));
    }
    let result = match fs::write(filepath.value.clone(), &data.value) {
        Ok(_) => {
            1
        },
        Err(_) => {
            //tok.print_custom_error(&format!("write_file() failed to write to file: {}", filepath.value));
            //panic!();
            0
        }
    };
    ast::Literal::number(result.to_string())
}

// Web
fn serve_static_folder(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    use rocket::config::{Config, Environment};
    use rocket_contrib::serve::StaticFiles;
    // Returns string Literal if success, null Literal if fail
    let folderpath = arguments.get(0).unwrap();
    let address = arguments.get(1).unwrap();
    let port = arguments.get(2).unwrap();
    if folderpath.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("serve_static_folder() expects 1st argument (folder_path) of type String, but received {:?} instead", folderpath.literal_type));
    }
    if address.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("serve_static_folder() expects 2nd argument (address) of type String, but received {:?} instead", address.literal_type));
    }
    if port.literal_type != ast::LiteralType::Number {
        tok.print_custom_error(&format!("serve_static_folder() expects 3rd argument (port) of type Number, but received {:?} instead", port.literal_type));
    }
    // Do some integer checks
    let port_float = Expr::string_to_float(&port);
    if port_float.fract() != 0.0 {
        tok.print_custom_error(&format!("{} is a float and is not a valid port for serve_static_folder(). Only positive integers are allowed", port_float));
    }
    let port_integer = port_float as i32;
    if port_integer < 0 {
        tok.print_custom_error(&format!("{} is negative and is not a valid port for serve_static_folder(). Only positive integers are allowed", port_float));
    }
    let port_integer = port_integer as u16;
    let config = match Config::build(Environment::Staging)
                .address(&address.value)
                .port(port_integer)
                .finalize() {
                    Ok(result) => result,
                    Err(_) => {
                        tok.print_custom_error(&format!("Either address or port of serve_static_folder() is invalid"));
                        panic!();
                    }
                };
                        
    let error = rocket::custom(config).mount("/", StaticFiles::from(&folderpath.value)).launch();
    println!("Launch failed! Error: {}", error);
    ast::Literal::none()
}

fn web_get(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns string Literal if success, null Literal if fail
    let url = arguments.get(0).unwrap();
    if url.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("web_get() expects one argument (url) of type String, but received {:?} instead", url.literal_type));
    }
    let result = match reqwest::blocking::get(&url.value) {
        Ok(content) => ast::Literal::string(content.text().unwrap()),
        Err(_) => {
            //tok.print_custom_error(&format!("web_get() failed to GET url: {}", url.value));
            //panic!();
            ast::Literal::null()
        }
    };
    return result;
}

use std::collections::HashMap;

fn web_post(arguments: Vec<ast::Literal>, tok: &token::Token) -> ast::Literal {
    // Returns string Literal if success, null Literal if fail
    let url = arguments.get(0).unwrap();
    let params = arguments.get(1).unwrap();

    if url.literal_type != ast::LiteralType::String {
        tok.print_custom_error(&format!("web_post() expects 1st argument (url) of type String, but received {:?} instead", url.literal_type));
    }
    if params.literal_type != ast::LiteralType::Array {
        tok.print_custom_error(&format!("web_post() expects 2nd argument (parameters) of type Array, but received {:?} instead", params.literal_type));
    }
    let original_array = &params.array_values;
    let length = original_array.len();
    if (length % 2) != 0 {
        tok.print_custom_error(&format!("web_post() expects 2nd argument (parameters) to have even length, but received length {:?} instead", length));
    }
    if original_array.len() > 0 {
        let array_type = original_array.get(0).unwrap().literal_type;
        if array_type != ast::LiteralType::String {
            tok.print_custom_error(&format!("web_post() expects 2nd argument (parameters) of type Array to have String elements, but received {:?} elements instead", array_type));
        }
    }
    let mut map = HashMap::new();
    let mut index = 0;
    while index < length {
        map.insert(original_array.get(index).unwrap().value.clone(), original_array.get(index + 1).unwrap().value.clone());
        index += 2;
    }
    let client = reqwest::blocking::Client::new();
    let result = match client.post(&url.value).json(&map).send() {
        Ok(content) => ast::Literal::string(content.text().unwrap()),
        Err(_) => {
            //tok.print_custom_error(&format!("web_post() failed to POST url: {}", url.value));
            //panic!();
            ast::Literal::null()
        }
    };
    return result;
}