/*
벡터는 여러 데이터 조각을 저장 할 수 있는 데이터 구조 입니다.
데이터는 동일한 자료형이어야 합니다.
숫자, 문자열, 열거형, 사용자 정의 Struct도 저장 할 수 있습니다.
벡터는 정보 목록에 사용됩니다. 예를 들어 식료품 목록이 있는 경우 모든 식료품 목록을 벡터에 저장 할 수 있습니다.
벡터를 사용하면 데이터를 추가하고 제거할 수 있습니다. 또한 데이터 작업을 위해 벡터의 항목을 쉽게 탐색 할 수 있습니다.

let my_numbers = vec![1,2,3];
let mut my_numbers = Vec::new();
my_numbers.push(1);
my_numbers.pop();
my_numbers.len();
*/

fn main(){
    let mut a = vec![1,2,3];
    a.push(4);
    println!("{:?}", a.len());
}