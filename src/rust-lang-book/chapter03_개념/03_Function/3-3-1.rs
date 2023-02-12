/*
함수 매개 변수

함수는 함수 고유한 부분인 특별한 변수 매개변수를 갖는 형식으로 선언 될 수 있습니다.
매개변수에는 반드시 타입을 지정해야 합니다.

 */

fn main(){
    another_function(5);
}

fn another_function(x: i32){
    println!("The value of x is : {}", x);
}