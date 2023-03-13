/*
구조체는 여러 데이터 조각을 포함하는 데이터 타입 생성을 도와줍니다.
struct는 structure의 줄임말로 여러 데이터 조각을 포함하는 데이터 타입입니다.

구조체에 포함 된 각 데이터 조각은 반드시 채워져 있어야 하며, 구조체의 일부만 갖고나 일부만 갖지 않을 수는 없습니다.
구조체에 포함된 각각의 데이터 조각을 필드라고 부릅니다.

구조체는 데이터로 작업하는 것을 수월하게 만들어 줍니다.
프로그램에서 유사한 데이터끼리 묶을 수 있고 코드의 다른 부분으로 함께 이동시킬 수 있기 때문입니다

*/ 

struct GroceryItem {
    stock : i32,
    price : f64,
}

#[derive(Default)]
struct UsingDefaultItem{
    test1 : i32,
    test2 : f64,
    test3 : i32
}

fn main(){
    let cereal = GroceryItem{
        stock : 10,
        price: 19.9
    };

    println!("stock : {:?}", cereal.stock);

    let default_item = UsingDefaultItem{
        test1 : 10,
        ..Default::default()
    };

    println!("test2 : {}", default_item.test2);

}
