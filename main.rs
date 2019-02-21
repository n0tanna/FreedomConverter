use std::io;

fn main (){
    println!("Temperature Converter");
    println!("*********************");
    println!("If you want to convert from C to F type in '1', for F to C type in '2'");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read from stdin");

    let choice: u32 = choice.trim().parse()
        .expect("Please type a number!");

    println!("Number {}", choice);

    if choice == 1 {
        conversion_to_f();
    } else if  choice == 2 {
        println!("do that");
    } else {
        println!("This");
    }

}

fn conversion_to_f() {

    println!("Please input the number you want to conver to F: ");
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: u32 = number.trim().parse()
        .expect("Please type a number!");

    let conversion = number

}