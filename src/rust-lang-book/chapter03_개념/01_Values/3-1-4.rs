/*
Shadowing

이전에 선언한 변수와 같은 이름으로 변수를 선언하면 새 변수는 이전 변수를 shadows하게 된다.

Rustaceans(러스트사용자들)은 이를 첫 변수가 두 번째에 의해 shadowed 됐다고 표현한다.


 */

 fn main(){
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is : {}", x);
 }