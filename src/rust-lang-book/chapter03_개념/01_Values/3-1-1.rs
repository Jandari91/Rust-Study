/*
rust에서 기본 변수는 불변성이다.

Err : cannot assign twice to immutable variable

*/

fn main(){
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // let x로 선언되어 있는 것은 고칠 수 없다.
    println!("The value of x is: {}", x);
}