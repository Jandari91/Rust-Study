/*
    문자열형은 이름과 단어와 같은 텍스트 정보를 저장하는데 사용됩니다.
    Rust에는 여러 유형의 문자열이 있는데, 가장 일반적으로 사용되는 두 가지 타입이 있습니다.
    * String - owned
    * &str - borrowed String slice

     대문자 S로 된 String 타입은 소유 데이터 타입으로 이것은 곧 중요한 차이가 됩니다.
     그리고 참조형인 &str이 있습니다. 이것은 빌림 문자열 slice입니다. string slice라고 불리웁니다.

    struct에 문자열 데이터를 저장하고 싶다면 소유 데이터 타입인 String을 사용해야 합니다.
    함수에 문자열 데이터를 제공하려 한다면 &str 문자열 슬라이스를 사용하는 것이 좋습니다.

    String은 자동적으로 빌림이됩니다. 따라서 .to_owned() 또는 String::from()을 사용하여 문자열 슬라이스의 복사본을 만들고 struct에서는 소유 데이터 타입인 String을 사용해 문자열을 저장합니다.

*/
struct LineItem {
    name : String,
    count : i32
}

fn print_name(name:&str) {
    println!("name : {:?}", name);
}

fn main(){
    let receipt = vec![
        LineItem {
            name : "cereal".to_owned(),
            count : 1
        },
        LineItem {
            name : String::from("fruit"),
            count : 3
        }
    ];
    
    for item in receipt {
        print_name(&item.name);
        println!("name : {:?}, count : {:?}", item.name, item.count);

    }
}