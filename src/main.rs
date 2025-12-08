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

    let mut i = 5;
    let mut v = 199;

    for x in 0..i {
        println!("{}", x + v);
    }

    while i != v {
        println!("i = {}, v = {}", i, v);
        if i > v {
            i = i - 1;
        } else {
            v = v - 1;
        }
    }
    if i == v {
        println!("i and v are equal at {}", i);
    }

    println!("done: i = {}, v = {}", i, v);
}



