/*
for와 함께하는 콜렉션 반복하기
아래와 같이 콜렉션의 각 요수에 대해 출력 할 수 있다.
*/

fn main(){
    let a = [10, 20, 30, 40, 50];
    for item in a.iter(){
        println!("the value is: {}", item);
    }

    for item in a.iter().rev() {
        println!("the value is: {}", item);
    }
}