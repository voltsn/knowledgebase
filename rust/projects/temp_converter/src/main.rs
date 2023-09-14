use std::io;

fn main() {
    println!("Fahrenheit to celsius converter.");
    println!("Type 'quit' to exit.");

    let mut temperature = String::new(); 

    loop {
        println!("");
        println!("Temperature: ");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature = to_celsius(temperature);
        println!("Temperature in celsius: {:.2} °C", temperature);
        break;
    }
}

fn to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0/9.0)
}
