use std::io::{self, Write};

fn check_even_odd(num: i64) {
    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}

fn safe_cast_to_i64(n: f64) -> Option<i64> {
    if n.is_finite() && n <= i64::MAX as f64 && n >= i64::MIN as f64 {
        Some(n as i64)
    } else {
        None
    }
}

fn mathematical_operations(v: f64, i: f64) {
    if i == 0.0 {
        println!("Can't divide or take remainder with 0 — pick another number.");
        return;
    }

    println!("---------------------------");
    println!("Addition: {}", v + i);
    println!("Subtraction: {}", v - i);
    println!("Multiplication: {}", v * i);
    println!("Division / Distance: {}", v / i);

    // Use integer remainder if possible
    if let (Some(v_int), Some(i_int)) = (safe_cast_to_i64(v), safe_cast_to_i64(i)) {
        println!("Remainder: {}", v_int % i_int);
        check_even_odd(v_int);
        check_even_odd(i_int);
    } else {
        println!("Cannot calculate remainder or even/odd for these numbers (out of range or invalid).");
    }
}

fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                match trimmed.parse::<f64>() {
                    Ok(num) if num.is_finite() => return num,
                    _ => println!("'{}' is not a valid finite number — try again.", trimmed),
                }
            }
            Err(err) => println!("Failed to read input: {}", err),
        }
    }
}

fn main() {
    let v = read_number("Enter first number (v): ");
    let i = read_number("Enter second number (i): ");

    println!("---------------------------");
    println!("Mathematical operations between {} and {}", v, i);
    mathematical_operations(v, i);
}
