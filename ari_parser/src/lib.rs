//#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;


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
use crate::ari_errors::SCRIPT;
use crate::ari_errors::BORDER_LENGTH;

pub fn get_version() -> String {
    return "Ari 0.1.0".to_owned();
}

pub fn run_script(script_name : &str){
    {
        // Block statement to ensure mutex is unlocked
        let script_ref : &mut bool = &mut SCRIPT.lock().unwrap();
        *script_ref = true;
    }
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
    
    let contents = match fs::read_to_string(script_name) {
        Ok(content) => content,
        Err(_) => {
            ari_errors::print_red("Error: ", false, true);
            ari_errors::print_white(&format!("{} does not exist.", script_name), false, true);
            ari_errors::exit();
            panic!();
        }
    };
    run(&contents, 1);
}

pub fn run_interpreter(){
    {
        // Block statement to ensure mutex is unlocked
        let script_ref : &mut bool = &mut SCRIPT.lock().unwrap();
        *script_ref = false;
    }
    let version = get_version();
    {
        // Block statement to ensure mutex is unlocked
        let len_ref : &mut usize = &mut BORDER_LENGTH.lock().unwrap();
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
        print!("\n> ");
        io::stdout().flush().unwrap();
        let mut input_line = String::new();
        match io::stdin().read_line(&mut input_line){
            Ok(_)=>{
                run(&input_line.trim(), line_number);
            },
            Err(e)=>{
                println!("Error!\n{}", e)
            }
        }
    }

}

pub fn run(input: &str, line_number: usize){
    let mut scanner_struct = scanner::Scanner::new(input, line_number);
    let tokens = scanner_struct.scan_tokens();
    let mut parser_struct = parser::Parser::new(tokens);
    let statements = parser_struct.parse();
    for s in statements {
        s.evaluate_statement();
    }
}