mod operators;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut numbers = Vec::new();
    for arg in &args {
        match arg.parse::<f64>() {
            Ok(n) => numbers.push(n),
            Err(_) => {
                eprintln!("Error: '{}' is not a valid number", arg);
                std::process::exit(1);
            }
        }
    }

    let numbers_as_string = args.join(", ");

    println!(
        "The addition of {} gives {}",
        numbers_as_string,
        operators::add(numbers)
    );
}

