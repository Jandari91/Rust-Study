/*
let 구문에서 if문 사용하기

if가 표현식이기 대문에 let 구문 우측에 사용 가능합니다.
*/

fn main(){
    let condition = true;
    let number = if condition {
        5
    } else{
        6
    };
    println!("number is : {}", number);
}