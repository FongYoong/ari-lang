use crate::token;
use crate::ast;
use crate::function as func;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref ENV: Mutex<EnvManager> = {
        let m = Mutex::new(EnvManager::new());
        // Add native functions
        let power = func::Function::new_native(func::NativeType::Power);
        m.lock().unwrap().get_env().define("power".to_string(), ast::Literal::new_function(power));
        let log = func::Function::new_native(func::NativeType::Log);
        m.lock().unwrap().get_env().define("log".to_string(), ast::Literal::new_function(log));
        let modulo = func::Function::new_native(func::NativeType::Modulo);
        m.lock().unwrap().get_env().define("modulo".to_string(), ast::Literal::new_function(modulo));
        let absolute = func::Function::new_native(func::NativeType::Absolute);
        m.lock().unwrap().get_env().define("absolute".to_string(), ast::Literal::new_function(absolute));
        let floor = func::Function::new_native(func::NativeType::Floor);
        m.lock().unwrap().get_env().define("floor".to_string(), ast::Literal::new_function(floor));
        let ceiling = func::Function::new_native(func::NativeType::Ceiling);
        m.lock().unwrap().get_env().define("ceiling".to_string(), ast::Literal::new_function(ceiling));
        let max = func::Function::new_native(func::NativeType::Max);
        m.lock().unwrap().get_env().define("max".to_string(), ast::Literal::new_function(max));
        let min = func::Function::new_native(func::NativeType::Min);
        m.lock().unwrap().get_env().define("min".to_string(), ast::Literal::new_function(min));

        let to_string = func::Function::new_native(func::NativeType::ToString);
        m.lock().unwrap().get_env().define("to_string".to_string(), ast::Literal::new_function(to_string));
        let to_number = func::Function::new_native(func::NativeType::ToNumber);
        m.lock().unwrap().get_env().define("to_number".to_string(), ast::Literal::new_function(to_number));

        let split = func::Function::new_native(func::NativeType::Split);
        m.lock().unwrap().get_env().define("split".to_string(), ast::Literal::new_function(split));
        let to_lowercase = func::Function::new_native(func::NativeType::ToLowercase);
        m.lock().unwrap().get_env().define("to_lowercase".to_string(), ast::Literal::new_function(to_lowercase));
        let to_uppercase = func::Function::new_native(func::NativeType::ToUpperCase);
        m.lock().unwrap().get_env().define("to_uppercase".to_string(), ast::Literal::new_function(to_uppercase));

        let length = func::Function::new_native(func::NativeType::Length);
        m.lock().unwrap().get_env().define("length".to_string(), ast::Literal::new_function(length));
        let insert = func::Function::new_native(func::NativeType::Insert);
        m.lock().unwrap().get_env().define("insert".to_string(), ast::Literal::new_function(insert));
        let remove = func::Function::new_native(func::NativeType::Remove);
        m.lock().unwrap().get_env().define("remove".to_string(), ast::Literal::new_function(remove));
        let map = func::Function::new_native(func::NativeType::Map);
        m.lock().unwrap().get_env().define("map".to_string(), ast::Literal::new_function(map));
        let filter = func::Function::new_native(func::NativeType::Filter);
        m.lock().unwrap().get_env().define("filter".to_string(), ast::Literal::new_function(filter));
        let reduce = func::Function::new_native(func::NativeType::Reduce);
        m.lock().unwrap().get_env().define("reduce".to_string(), ast::Literal::new_function(reduce));
        let range = func::Function::new_native(func::NativeType::Range);
        m.lock().unwrap().get_env().define("range".to_string(), ast::Literal::new_function(range));
        let linspace = func::Function::new_native(func::NativeType::Linspace);
        m.lock().unwrap().get_env().define("linspace".to_string(), ast::Literal::new_function(linspace));
        let repeat = func::Function::new_native(func::NativeType::Repeat);
        m.lock().unwrap().get_env().define("repeat".to_string(), ast::Literal::new_function(repeat));

        let random_choose = func::Function::new_native(func::NativeType::RandomChoose);
        m.lock().unwrap().get_env().define("random_choose".to_string(), ast::Literal::new_function(random_choose));
        let random_normal = func::Function::new_native(func::NativeType::RandomNormal);
        m.lock().unwrap().get_env().define("random_normal".to_string(), ast::Literal::new_function(random_normal));

        let read_file = func::Function::new_native(func::NativeType::ReadFile);
        m.lock().unwrap().get_env().define("read_file".to_string(), ast::Literal::new_function(read_file));
        let write_file = func::Function::new_native(func::NativeType::WriteFile);
        m.lock().unwrap().get_env().define("write_file".to_string(), ast::Literal::new_function(write_file));

        let serve_static_folder = func::Function::new_native(func::NativeType::ServeStaticFolder);
        m.lock().unwrap().get_env().define("serve_static_folder".to_string(), ast::Literal::new_function(serve_static_folder));
        let web_get = func::Function::new_native(func::NativeType::WebGet);
        m.lock().unwrap().get_env().define("web_get".to_string(), ast::Literal::new_function(web_get));
        let web_post = func::Function::new_native(func::NativeType::WebPost);
        m.lock().unwrap().get_env().define("web_post".to_string(), ast::Literal::new_function(web_post));

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