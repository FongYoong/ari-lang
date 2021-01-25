//#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::fs;
use std::io;
use std::io::Write;
mod token;
mod ast;
mod scanner;
mod parser;
mod environment;
mod function;
use ari_errors;
use crate::ari_errors::BORDER_LENGTH;

/*
pub fn print_version(){
    let version = "0.1.0";
    ari_errors::print_white(&format!("Ari {}", version), true, true);
}
*/
pub fn get_version() -> String {
    return "Ari 0.1.0".to_owned();
}

pub fn run_script(script_name : &str){
    let version = get_version();
    let running = format!("Running {}:", script_name);
    let length = std::cmp::max(version.len(), running.len());
    {
        // Block statement to ensure mutex is unlocked
        let len_ref : &mut usize= &mut BORDER_LENGTH.lock().unwrap();
        *len_ref = length;
    }
    let upper = (0..length).map(|_| "‾").collect::<String>();
    let lower = (0..length).map(|_| "_").collect::<String>();
    ari_errors::print_green(&upper, true, true);
    ari_errors::print_white(&version, true, true);
    ari_errors::print_white(&running, true, false);
    ari_errors::print_green(&lower, true, true);
    ari_errors::print_white("\n", false, false);
    /*
    let contents = match fs::read_to_string(script_name) {
        Ok(content) => content,
        Err() => 
    }*/

    // Remember to handle file missing error gracefully!
    let contents = fs::read_to_string(script_name).expect("Something went wrong reading the file");

    //println!("{}", contents.as_bytes()[3]);
    //println!("{}", contents.chars().count());
    //println!("{}", contents.chars().nth(3).unwrap() == '\r');
    
    run(&contents, 1);
    /*
    for (line_number, line) in contents.split("\n").enumerate(){
        run(&line, 1);
        //println!("Line {}: {}", line_number + 1, line);
        /*
        match parse_input(&line) {
            Ok(_)=>{
                
            },
            Err(e)=>{
                println!("Error at line {} \n{}", line_number, e);
            }
        }
        */
    }*/
}

pub fn run_interpreter(){
    let version = get_version();
    {
        // Block statement to ensure mutex is unlocked
        let len_ref : &mut usize= &mut BORDER_LENGTH.lock().unwrap();
        *len_ref = version.len();
    }
    let upper = (0..version.len()).map(|_| "‾").collect::<String>();
    let lower = (0..version.len()).map(|_| "_").collect::<String>();
    ari_errors::print_green(&upper, true, true);
    ari_errors::print_white(&version, true, true);
    ari_errors::print_green(&lower, true, true);
    ari_errors::print_white("", false, false);
    let mut line_number = 0;
    loop{
        line_number += 1;
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line){
            Ok(_)=>{
                //println!("{}", &input_line.trim());
                run(&input_line.trim(), line_number);
            },
            Err(e)=>{
                println!("Error!\n{}", e)
            }
        }
    }

}

pub fn run(input: &str, line_number: usize){
    //println!("{}", input);
    // mut env_struct = environment::Environment::new();
    let mut scanner_struct = scanner::Scanner::new(input, line_number);
    let tokens = scanner_struct.scan_tokens();
    let mut parser_struct = parser::Parser::new(tokens);
    let statements = parser_struct.parse();
    for mut s in statements {
        s.evaluate_statement();
        /*
        match s.statement_type {
            ast::StatementType::Let => {
                if s.expr.unwrap().expr_type == ast::ExprType::None {
                    panic!("Uninitialised variable!");
                }
                //env_struct.define(s.token_name, literal)
            },
            _ => {

            }

        }
        */
        
    }
    /*
    let (literal_type, string_value) = expression.evaluate_expr();
    println!("{:?}, {}", literal_type, string_value);
    match literal_type {
        ast::LiteralType::Number => {
            
        },
        ast::LiteralType::String => {

        },
        ast::LiteralType::Bool | ast::LiteralType::Null => {

        },
        _ => {

        }
    };
    */

    /*
    match header.get(0) {
        None => Err("invalid header length"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version"),
    }
    */
}