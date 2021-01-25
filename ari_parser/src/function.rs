use crate::token;
use crate::ast;
use crate::environment::Environment;
use crate::environment::ENV;
use rayon::prelude::*; // For array operations/fast parallelism

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
    Clock, //Print current time

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
        let number_of_args = match native_type {
            NativeType::Clock => {
                0
            },
            _ => {
                panic!("new_native() has not accounted for {:?}", native_type);
            }
        };
        Function::new(FunctionType::Native, Vec::<token::Token>::with_capacity(number_of_args), None, native_type, None, token::Token::none())
    }
    pub fn none() -> Function {
        Function::new(FunctionType::None, Vec::<token::Token>::new(), None, NativeType::None, None, token::Token::none())
    }

    pub fn arg_length(&mut self) -> usize {
        self.arguments.len()
    }

    pub fn call(&mut self, arguments: Vec<ast::Literal>) -> Option<ast::Literal> {
        
        let result = match self.function_type {
            FunctionType::UserDefined => {
                //println!("Invoke user! {}", self.arguments.len());
                ENV.lock().unwrap().add_env(self.closure_env.as_mut().unwrap().clone());
                
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
                let r = Some(self.call_native(arguments));
                ENV.lock().unwrap().destroy_env();
                r
            },
            _ => {
                None
            }
        };
        return result;
    }

    pub fn call_user(&mut self, arguments: Vec<ast::Literal>) -> ast::Literal {
        for i in 0..arguments.len() {
            // Insert arg name: arg value into new scope
            ENV.lock().unwrap().get_env().define(self.arguments.get(i).unwrap().lexeme.to_string(), arguments.get(i).unwrap().clone());
        }
        return self.user_defined.as_mut().unwrap().evaluate_statement()
        //ast::Literal::none()
    }

    pub fn call_native(&mut self, arguments: Vec<ast::Literal>) -> ast::Literal {
        match self.native_type {
            NativeType::Clock => {
                clock()
            },
            _ => {
                panic!("call_native() has not accounted for {:?}", self.native_type);
            }
        }
    }
}

fn clock() -> ast::Literal {
    ast::Literal::string("Clock!!!".to_string())
}