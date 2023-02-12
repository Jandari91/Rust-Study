/*
구문과 표현식

rust는 let x= (let y = 6); 과 같이 사용할 수 없습니다.

반환값이 없기 때문입니다.

C나 C++은 x = y = 6을 사용 할 수 있지만 Rust는 사용 할 수 없습니다.

*/

fn main(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x : {}, y : {}", x, y);
}