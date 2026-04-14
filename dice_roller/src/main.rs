use std::io;


fn string_input(prompt: String) -> String {
    loop {
        let mut input = String::new();

        println!("{}", prompt);

        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read: {}", e);
            continue;
        }

        let trimmed = input.trim().to_string();

        if trimmed.is_empty() {
            eprintln!("Invalid input, please enter a valid string...");
            continue;
        }
        return trimmed;
    }
}


fn main() {
    /*
    define goal: create a system that takes in an input like 1d6 or 2d8
    and give a number output, and the give the dices totals...
    */
    let name = string_input(String::from("Enter your name: "));
    println!("Your name is: {}", name);
}