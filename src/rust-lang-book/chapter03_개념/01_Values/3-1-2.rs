/*
3-1-1.rs를 컴파일 통과하기 위해서는 아래와 같이 수정해야 한다.
*/

fn main(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // let x로 선언되어 있는 것은 고칠 수 없다.
    println!("The value of x is: {}", x);
}