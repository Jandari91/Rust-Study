/*
https://www.acmicpc.net/problem/9498
 */

use std::io;
fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let x = str.trim().parse::<i32>().unwrap();

    if x >= 90 && x <= 100{
        println!("A");
    }else if x >= 80 && x <= 89{
        println!("B");
    }else if x >= 70 && x <= 79 {
        println!("C");
    }else if x >= 60 && x <= 69 {
        println!("D");
    }else{
        println!("F");
    }
}