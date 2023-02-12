/*
https://www.acmicpc.net/problem/2588
*/

use std::io;

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();

    let n = n.trim().parse::<i32>().unwrap();
    let m = m.trim().parse::<i32>().unwrap();


    println!("{}", n * (m%10));
    println!("{}", n * ((m/10)%10));
    println!("{}", n * ((m/100)%10));
    println!("{}", n * m);
}