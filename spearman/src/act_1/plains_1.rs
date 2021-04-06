use lib_spearman::generate_pauses;
use std::io::{stdin, Read};

pub fn plains_1() {
    println!("Welcome to spearman, Press enter to continue");
    let welcome: &str = "... _..... _are you there _....";
    generate_pauses(welcome);
    if respond_nod() {
        println!("you said yes");
    } else {
        println!("nah")
    }
}

pub fn respond_nod() -> bool {
    println!("Y/N");
    let mut response = String::new();
    stdin()
        .read_to_string(&mut response)
        .expect("Failed to receive");
    println!("You guessed {}", response);
    println!("here");
    match response.as_str() {
        "y" => true,
        "n" => false,
        _ => false,
    }
}
