/*
Rust는 타입이 고정되어 있는 언어입니다. 그러니 어떤 현태의 데이터인지 명시하여 Rust에게 알려줘야 합니다.

타입은 크게 스칼라와 컴파운드로 나눠집니다.

*/

fn main(){
    let guess:u32 = "42".parse().expect("Not a number");
    println!("guess is : {}", guess);
}