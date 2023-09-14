use std::io;

fn main() {
    let mut number = String::new();

    println!("Number: ");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read loine");
    
    let number: i32 = number.trim()
        .parse()
        .expect("A number was expected");

    let fibo_number = fibo(number);

    println!("The {number} number is: {fibo_number}");
}

fn fibo(x: i32) -> i32 {

    if x <= 0 {
        return 0;
    }

    if x == 1 {
        return 1;
    }

    fibo(x-1) + fibo(x-2)
}
