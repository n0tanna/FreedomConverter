use std::io;

fn main (){
    println!("Freedom Converter");
    println!("*********************");
    println!("What do you want to convert?(Choose a number)\n 1.Speed \n 2.Length \n 3.Temperature");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read from stdin");

    let choice: u32 = choice.trim().parse()
        .expect("Please type a number!");

    println!("Number {}", choice);

    if choice == 1 {
        conversion_to_f();
    } else if  choice == 2 {
        conversion_to_c();
    } else if choice == 3 {
        temperature();
    } else {
        println!("That is not an option.");
    }

}

fn temperature() {

    println!("Choose one of the following: \n 1. Conversion to F from C \n 2. Conversion to C from F");
    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read from stdin");

    let choice: u32 = choice.trim().parse()
        .expect("Please type a number!");

    println!("Number {}", choice);

    if choice == 1 {
        conversion_to_f();
    } else if  choice == 2 {
        conversion_to_c();
    } else {
        println!("That is not an option.");
    }
}

fn conversion_to_f() {

    println!("Please input the number you want to convert to F: ");
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = (number * 1.8) + 32.0;

    println!("Your conversion is: {}", conversion);

}

fn conversion_to_c() {

    println!("Please input the number you want to convert to C: ");
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = (number - 32.0)/1.8;

    println!("Your conversion is: {}", conversion);
}