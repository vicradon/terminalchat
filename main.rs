use std::process;
use terminalchat::user;

fn arg_extract(command: &str, args: Vec<String>) -> Result<(String, String), &str> {
    match command {
        "login" => {
            if args.len() != 4 {
                return Err("Less than 4 arguments provided");
            }
            if args[0] != "--username" {
                return Err("username arg not found");
            }
            if args[1].is_empty() {
                return Err("No username supplied");
            }
            if args[2] != "--password" {
                return Err("password arg not found");
            }
            if args[3].is_empty() {
                return Err("No password supplied");
            }
            
            Ok((args[1].clone(), args[3].clone()))
        },
        "register" => {
            if args.len() != 4 {
                return Err("Less than 4 arguments provided");
            }
            if args[0] != "--username" {
                return Err("username arg not found");
            }
            if args[1].is_empty() {
                return Err("No username supplied");
            }
            if args[2] != "--password" {
                return Err("password arg not found");
            }
            if args[3].is_empty() {
                return Err("No password supplied");
            }
            
            Ok((args[1].clone(), args[3].clone()))
        },
        &_ => todo!()
    }
}

fn main(){
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("No args supplied. Exiting...");
        std::process::exit(1);
    }

    let command = args[1].to_string();
    let other_args = args[2..].to_vec();

    match command.as_str() {
        "login" => {
            match arg_extract("login", other_args) {
                Ok((username, password)) => {
                    user::login(&username, &password);
                },
                Err(err_val) => eprintln!("{}", err_val),
            };
        },
        "register" => {
            match arg_extract("register", other_args) {
                Ok((username, password)) => {
                    user::register(&username, &password);
                },
                Err(err_val) => {
                    eprintln!("{}", err_val);
                    process::exit(1);
                },
            };
        },
        &_ => todo!()
    }

    println!("{:?}", args);
}
