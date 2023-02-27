/*
https://www.acmicpc.net/problem/2480
 */

use std::io;

fn main(){
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    let numbers:Vec<&str> = str.split_whitespace().collect();
    let a = numbers[0].parse::<i32>().unwrap() -1;
    let b = numbers[1].parse::<i32>().unwrap() -1;
    let c = numbers[2].parse::<i32>().unwrap() -1;

    let mut arr:[i32;6] = [0,0,0,0,0,0];
    arr[a as usize] = arr[a as usize] + 1;
    arr[b as usize] = arr[b as usize] + 1;
    arr[c as usize] = arr[c as usize] + 1;
    let mut max = 1;
    let mut index = 0;

    for n in 0 .. 6 {
        if arr[n] > max {
            max = arr[n];
            index = n;
        } else if max == 1 && arr[n] == 1{
            index = n;
        }

    }

    if max == 3 {
        println!("{}", 10000+((index+1) * 1000));
    }else if max == 2 {
        println!("{}", 1000+((index+1) * 100));
    }else{
        println!("{}", ((index+1) * 100));
    }

}