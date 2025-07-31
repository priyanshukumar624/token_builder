use std::io::{self, Write};

fn main() {
    let token = ask_input("Enter token: ");
    let encrypted_user_id = ask_input("Enter encryptedUserId: ");
    let mobile_number = ask_input("Enter mobile number: ");

    let formatted = format!(" {}  userid  {}  user  {}", token, encrypted_user_id, mobile_number);

    println!("\nFormatted Output:\n{}", formatted);
}

fn ask_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string() 
}
