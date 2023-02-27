/*
https://www.acmicpc.net/problem/2753
 */

use std::io;

fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let x = str.trim().parse::<i32>().unwrap();

    if x % 4 ==0 && x % 100 != 0 {
        println!("1");
    }else if x % 400 == 0 {
        println!("1")
    }else{
        println!("0");
    }
}