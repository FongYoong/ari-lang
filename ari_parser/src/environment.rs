use crate::token;
use crate::ast;
use crate::function as func;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref ENV: Mutex<EnvManager> = {
        let m = Mutex::new(EnvManager::new());
        // Add native functions
        let clock_function = func::Function::new_native(func::NativeType::Clock);
        m.lock().unwrap().get_env().define("clock".to_string(), ast::Literal::new_function(clock_function));
        m
    };
}

pub struct EnvManager{
    envs: Vec<Environment>,
}
impl EnvManager {
    pub fn new() -> EnvManager {
        EnvManager {
            envs : vec![Environment::new()],
        }
    }
    
    pub fn get_env(&mut self) -> &mut Environment {
        let index = self.envs.len() - 1;
        return &mut self.envs[index]
    }
    pub fn get_nth_env(&mut self, index:usize) -> &mut Environment {
        return &mut self.envs[index];
    }
    pub fn add_env(&mut self, env: Environment) {
        self.envs.push(env);
    }
    pub fn create_env(&mut self) {
        self.envs.push(Environment::new());
    }
    pub fn destroy_env(&mut self) {
        let final_length = self.envs.len().saturating_sub(1);
        self.envs.truncate(final_length);
    }
    
    pub fn get_variable(&mut self, token_key: &token::Token) -> ast::Literal {
        let mut len = self.envs.len();
        while len > 0 {
            match self.get_nth_env(len - 1).get(token_key) {
                Ok(literal) => {
                    return literal.clone();
                },
                Err(_) =>{
                    len -= 1;
                }
            }
        }
        token_key.print_custom_error(&format!("'{}' is an undefined variable", token_key.lexeme));
        panic!()
    }

    pub fn assign_variable(&mut self, tok : &token::Token, literal_value : ast::Literal) {
        let mut len = self.envs.len();
        while len > 0 {
            let env = self.get_nth_env(len - 1);
            if env.contains_key(tok) {
                env.define(tok.lexeme.to_owned(), literal_value);
                return;
            }
            else {
                len -= 1;
            }
        }
        tok.print_custom_error(&format!("'{}' variable cannot be found in this scope", tok.lexeme));
    }
}

pub struct Environment{
    pub values: HashMap<String, ast::Literal>,
}
impl Clone for Environment {
    fn clone(&self) -> Environment {
        Environment {
            values: self.values.clone(),
        }
    }
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            values : HashMap::<String, ast::Literal>::new(),

        }
    }

    pub fn define(&mut self, key: String, value: ast::Literal) {
        // IMPORTANT
        // Only use define() to create new variables.
        // For assignment/redefinition, use EnvManager's assign_variable() instead.
        self.values.insert(key, value);
    }

    pub fn contains_key(&mut self, token_key: &token::Token) -> bool {
        return self.values.contains_key(&token_key.lexeme);
    }

    pub fn get(&mut self, token_key: &token::Token) -> Result<ast::Literal, &str> {
        match self.values.get(&token_key.lexeme) {
            Some(literal) => {
                Ok(literal.clone())
            },
            _ => {
                Err("")
            }
        }
    }
}