/*
if 표현식

다른 언어와 마찬가지로 다른 부분이 별로 없습니다.

*/

fn main(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 ==0 {
        println!("number is divisible by 3");
    } else if number % 2 ==0 {
        println!("number is divisible by 2");
    } else{
        println!("number is not divisible by 4, 3, or 2");
    }

}