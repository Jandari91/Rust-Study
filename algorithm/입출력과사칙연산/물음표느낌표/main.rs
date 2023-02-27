/*
https://www.acmicpc.net/problem/10926
*/

use std::io;

fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let output = str.trim();
    println!("{}??!", output);
}