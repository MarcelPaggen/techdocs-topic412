use std::io;

fn main() {
    println!("Factorial calculator");

    loop {
        println!("Please input the number to calculate the factorial: ");

        let mut number_string = String::new();

        io::stdin().read_line(&mut number_string)
            .expect("Failed to read line");

        if number_string == "exit\n" {break;}

        let to_calculate: i64 = match number_string.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = calculate_factorial(to_calculate);
 
        println!("The factorial is: {}", result);

    }    
}

fn calculate_factorial(value: i64) -> i64 {
    if value == 0 { 
        1
    }
    else {
        value * calculate_factorial(value - 1)
    }
}