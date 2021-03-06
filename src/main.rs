use std::io;
use multifactorials::Multifactorials;

#[cfg(target_os = "windows")]
use clear::clear;

#[cfg(target_os = "linux")]
use clear::clear;

mod clear;

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
            let number = get_user_input("Enter a number to calculate its factorial: ").parse::<f64>().unwrap();
            let res = Multifactorials::simple(number);

            clear();

            println!("{}! = {}", number, res);
        },
        "complex" => {
            let number = get_user_input("Enter a number to calculate its factorial: ").parse::<f64>().unwrap();

            clear();

            let number_of_iterations = get_user_input("Enter the number of iterations: ").parse::<i32>().unwrap();

            clear();

            let res = Multifactorials::complex(number, number_of_iterations);

            println!("{}{} = {}", number, "!".repeat(number_of_iterations as usize), res);
        },
        _ => {
            println!("Invalid input");
        }
    }
}