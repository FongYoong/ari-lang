use std::env;
use ari_parser;
use ari_errors;

fn main() {
    env::set_var("ROCKET_CLI_COLORS", "off");
    let args: Vec<String> = env::args().collect();
    let arg_length = args.len();
    match arg_length {
        1 =>{
            ari_parser::run_interpreter();
        },
        2 =>{
            ari_parser::run_script(&args[1])
        },
        _ =>{
            println!("Too many arguments!\nUsage: ari [script_name]")
        }
    }
    ari_errors::exit();
}

