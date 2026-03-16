use std::{io, process::exit};

fn main() {
    println!("1. fa: convert C to F");
    println!("2. cel: convert F to C");
    println!("please enter the mode:");
    
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("unexpected input!");

    let mode = mode.trim();

    let mut temp = String::new();

    println!("Please enter the temp:");
    io::stdin()
        .read_line(&mut temp)
        .expect("unexpected input!");

    let temp = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("unexpected input");

            exit(-1);
        }
    };

    if mode == "1" {
        let result = c_to_f(temp);
        println!("the result is {result}");
    }
    else if mode == "2" {
        let result = f_to_c(temp);
        println!("the result is {result}");
    }
}

fn f_to_c(f: f64) -> f64{
    (f-32.0)/1.8
}
fn c_to_f(c: f64) -> f64{
    c*1.8 + 32.0
}