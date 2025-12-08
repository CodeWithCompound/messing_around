fn print_name(n: &String) {
    println!("Borrowed: {}", n);
}

fn main() {
    // ownership example
    let a = String::from("rust");
    /*
    ----------------------
    bad example:
        
    let b = a; 
        
    this will explode because of ownership rules
    ----------------------
     */

    // good example:
    let b = a.clone(); // deep copy
    println!("{}", b);
    println!("{}", a);

    // borrowing example
    let name = String::from("Alice");
    print_name(&name);
    println!("Not borrowed: {}", name);
}


