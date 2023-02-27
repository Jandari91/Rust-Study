/*
https://www.acmicpc.net/problem/14681
 */

use std::io;

fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let x = str.trim().parse::<i32>().unwrap();

    let mut arr = String::new();
    io::stdin().read_line(&mut arr).unwrap();
    let y = arr.trim().parse::<i32>().unwrap();

    if x < 0 && y < 0 {
        println!("3");
    }else if x < 0 && y > 0 {
        println!("2");
    }else if x > 0 && y < 0 {
        println!("4");
    }else{
        println!("1");
    }
}