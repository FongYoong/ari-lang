use std::process;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SCRIPT: Mutex<bool> = {Mutex::new(true)}; // Check if running script or interpreter
    pub static ref BORDER_LENGTH: Mutex<usize> = Mutex::new(0);
    //pub static ref IS_WEB: bool = false; // A reminder of possbily using WASM for running on the web
}

// For colourful terminal
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug)]
pub enum ErrorType{

    // Scanner
    UnknownToken,
    ConsumeStringLexeme,

    // Parser
    ExpectExpression,
    ExpectRightBracket,
    ExpectLeftParen,
    ExpectRightParen,
    ExpectLeftBrace,
    ExpectRightBrace,
    ExpectSemicolon,
    ExpectVariableName,
    ExpectFunctionName,
    ExpectClassName,
    ExpectArgumentName,
    InvalidAssignment,
    InvalidForLoop,
    TooManyArguments,
    NoArrayAccessIndex,
    ArrayAccessComma,

    // evaluate_statement() in ast.rs
    InvalidVariableDefinition,

    // evaluate_expr() in ast.rs
    EvalExprBinary,
    EvalExprUnary,
    EvalExprGrouping,



}

pub fn print_error(context:ErrorType, source:&str, index:usize, line_number:usize,){
    let error_name = match context{

        // Scanner
        ErrorType::UnknownToken => {
            "Error parsing (GetChar)"
        },
        ErrorType::ConsumeStringLexeme => {
            "Error parsing (Unterminated string)"
        },

        // Parser
        ErrorType::ExpectExpression => {
            "Expect expression"
        },
        ErrorType::ExpectRightBracket => {
            "Expect ']' after expression"
        },
        ErrorType::ExpectLeftParen => {
            "Expect '(' after expression"
        },
        ErrorType::ExpectRightParen => {
            "Expect ')' after expression"
        },
        ErrorType::ExpectLeftBrace => {
            "Expect '{' after expression"
        },
        ErrorType::ExpectRightBrace => {
            "Expect '}' after expression"
        },
        ErrorType::ExpectSemicolon => {
            "Expect ';' after expression"
        },
        ErrorType::ExpectVariableName => {
            "Expect variable name after 'let'"
        },
        ErrorType::ExpectFunctionName => {
            "Expect function name after 'fn'"
        },
        ErrorType::ExpectClassName => {
            "Expect class name after 'class"
        },
        ErrorType::ExpectArgumentName => {
            "Expect argument name"
        },
        ErrorType::InvalidAssignment => {
            "Invalid assignment"
        },
        ErrorType::InvalidForLoop => {
            "Invalid 'for' loop format"
        },
        ErrorType::TooManyArguments => {
            "Only up to 255 arguments are allowed"
        },
        ErrorType::NoArrayAccessIndex => {
            "Array access index not specified"
        },
        ErrorType::ArrayAccessComma => {
            "Unwanted comma found at array index"
        },

        // evaluate_statement() in ast.rs
        ErrorType::InvalidVariableDefinition => {
            "Invalid variable definition"
        },

        // evaluate_expr() in ast.rs
        ErrorType::EvalExprBinary => {
            "Expect ')' after expression"
        },
        ErrorType::EvalExprUnary => {
            "Expect ')' after expression"
        },
        ErrorType::EvalExprGrouping => {
            "Expect ')' after expression"
        },
        
        _ => {
            panic!("{:?} not implemented in ari_errors.", context);
        }

    };
    print_custom_error(error_name, source, index, line_number);
}

pub fn print_custom_error(message:&str, source:&str, index:usize, line_number:usize){
    let line_number_len = line_number.to_string().len();
    let left_spacing = format!("     {} |", (0..line_number_len).map(|_| " ").collect::<String>());
    let pointer_spacing = (0..index - 1).map(|_| " ").collect::<String>();
    print_red("\nError: ", false, true);
    print_white(&format!(": {} at line {}\n{}", message, line_number, left_spacing), true, true);
    print_yellow(&format!("{} {}", "Line", line_number), false, true);
    print_white(&format!(" |\t{}\n{}\t{}^", source, left_spacing, pointer_spacing), true, true);
    exit();
    // Make sure to print in white before exiting.
    // Otherwise, the terminal colour is permanently affected even after the program exits.
}

pub fn print_simple_error(message: &str) {
    println!("{}", message);
    exit();
}

pub fn print_white(s: &str, newline: bool, bold: bool) {
    print_colour(s, Color::White, newline, bold);
}

pub fn print_green(s: &str, newline: bool, bold: bool) {
    print_colour(s, Color::Green, newline, bold);
}

pub fn print_red(s: &str, newline: bool, bold: bool) {
    print_colour(s, Color::Red, newline, bold);
}
pub fn print_yellow(s: &str, newline: bool, bold: bool) {
    print_colour(s, Color::Yellow, newline, bold);
}

pub fn print_colour(s: &str, color: Color, newline: bool, bold: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(color));
    if bold {
        spec.set_bold(true);
    }
    stdout.set_color(&spec).unwrap();
    if newline {
        writeln!(&mut stdout, "{}", s).unwrap();
    }
    else {
        write!(&mut stdout, "{}", s).unwrap();
    }
}

use std::io;
pub fn exit() {
    let len_ref : &usize = &BORDER_LENGTH.lock().unwrap();
    let lower = (0..*len_ref).map(|_| "_").collect::<String>();
    println!("");
    print_green(&lower, true, true);
    print_white("", false, false);
    let script_ref : &bool = &SCRIPT.lock().unwrap();
    if !script_ref {
        // Is running interpreter
        println!("Press any key to exit.");
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line);
    }
    process::exit(0);
}