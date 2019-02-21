use std::io;

fn main (){
    println!("Freedom Converter");
    println!("*********************");
    println!("What do you want to convert?(Choose a number)\n 1.Speed \n 2.Length \n 3.Temperature");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read from stdin");

    let choice: u32 = choice.trim().parse()
        .expect("Please type a number!");

    println!("Number {}", choice);

    if choice == 1 {
        speed();
    } else if  choice == 2 {
        length();
    } else if choice == 3 {
        temperature();
    } else {
        println!("That is not an option.");
        main();
    }

}

fn restart(){
    println!("Would you like to convert something else? Y/N.");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect ("failed to read from stdin");

    if choice.trim() == "Y"{
        main();
    } else if choice.trim() == "N"{
        println!("Thanks for using this converter!");
    } else {
        println!("That is not an option.");
        restart();
    }

}

fn length(){

    println!("Choose one of the following: \n 1. Conversion from meters to feet \n 2. Conversion from feet to meters");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read from stdin");

    let choice: u32 = choice.trim().parse()
        .expect("Please type a number!");

    println!("Number {}", choice);

    if choice == 1 {
        conversion_to_fe();
    } else if choice == 2 {
        conversion_to_me();
    } else {
        println!("That is not an option.");
        length();
    }
}

fn conversion_to_fe() {
    println!("Please input the number you want to convert to feet: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = number * 3.281;

    println!("Your conversion is: {}{}", conversion, " feet.");
    restart();

}

fn conversion_to_me() {
    println!("Please input the number you want to convert to meters: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = number / 0.3048;

    println!("Your conversion is: {}{}", conversion, " meters.");
    restart();

}

fn speed() {
    println!("Choose one of the following: \n 1. Conversion from kph to mph \n 2. Conversion from mph to kph");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read from stdin");

    let choice: u32 = choice.trim().parse()
        .expect("Please type a number!");

    println!("Number {}", choice);

    if choice == 1 {
        conversion_to_m();
    } else if choice == 2 {
        conversion_to_k();
    } else {
        println!("That is not an option.");
        speed();
    }
}

fn conversion_to_m() {

    println!("Please input the number you want to convert to miles: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = number / 1.609;

    println!("Your conversion is: {}{}", conversion, " mph.");
    restart();

}

fn conversion_to_k() {

    println!("Please input the number you want to convert to kilometers: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = number * 1.609;

    println!("Your conversion is: {}{}", conversion, " kph.");
    restart();
}

fn temperature() {

    println!("Choose one of the following: \n 1. Conversion to F from C \n 2. Conversion to C from F");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read from stdin");

    let choice: u32 = choice.trim().parse()
        .expect("Please type a number!");

    if choice == 1 {
        conversion_to_f();
    } else if  choice == 2 {
        conversion_to_c();
    } else {
        println!("That is not an option.");
        temperature();
    }
}

fn conversion_to_f() {

    println!("Please input the number you want to convert to F: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = (number * 1.8) + 32.0;

    println!("Your conversion is: {}{}", conversion, "F.");
    restart();
}

fn conversion_to_c() {

    println!("Please input the number you want to convert to C: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: f64 = number.trim().parse()
        .expect("Please type a number!");

    let conversion:f64 = (number - 32.0)/1.8;

    println!("Your conversion is: {}{}", conversion, "C.");
    restart();
}