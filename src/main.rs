// $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

use std::io;

fn main(){
    println!("Enter your name");   

    let mut name = String::new();

    io::stdin().read_line( &mut name).expect("Invalid input");
    println!("enter you age");

    let mut age=String::new();

    io::stdin().read_line(&mut age).expect("Invalid input");
    let age:u8=age.trim().parse().expect("value is not int");

    println!("Hello {name} Your age is: {age}");
}
