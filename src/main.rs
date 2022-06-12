use std::{io, process::Command};

fn clear() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cls")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("clear")
            .output()
            .expect("failed to execute process")
    };

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn get_user_input(question: &str) -> String {
    println!("{}", question);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<String>().unwrap()
}

fn main() {
    clear();

    let factorial_type = get_user_input("What type of factorial do you want to calculate? Simple / Complex\n");

    clear();

    match factorial_type.to_lowercase().as_str() {
        "simple" => {
            todo!()
        },
        "complex" => {
            todo!()
        },
        _ => {
            println!("Invalid input");
        }
    }
}