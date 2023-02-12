/*
https://www.acmicpc.net/problem/18108
*/

use std::io;
fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let y:i32 = str.trim().parse::<i32>().unwrap();
    println!("{}", y - 543);
}