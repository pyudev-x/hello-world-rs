use std::io;

fn main() {
    println!("Hello, world!");

    let name = input("Enter your name: ");

    println!("Hello, {}!", name.trim_end());
}


fn input(s:&str) -> String {
    let mut val = String::new();
    
    eprint!("{}", s);

    io::stdin().read_line(&mut val).expect("Could not read input.");


    return val;
}