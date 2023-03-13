/*
소유권은 코드의 성능을 향상시키고 다양한 상황에서도 올바르게 작동하도록 컴파일 될 수 있도록 합니다.

## managing memory
모든 프로그램은 항상 메모리 사용을 추적해야 하며 이에 실패하면 메모리 누수가 발생합니다.
메모리 누수는 프로그램에서 사용되는 메모리 추적에 실패해서 새로운 메모리를 예약해야 하는 경우를 뜻합니다.

모든 프로그래밍 언어들은 메모리를 관리하는 각자의 방법이 있습니다.
Rust는 소유권이라는 것을 이용하며 메모리의 소유자가 메모리를 정리하게 됨을 의미합니다.
Rust에서 소유자란 단순히 함수입니다.
메모리는 소유자로 부터 이동(moved)하거나 대여(borrowed)할 수 있습니다.


메모리는 누수를 방지하기 위해 어떻게든 관리가 되어야 하는데 Rust는 소유권을 이용해 메모리를 관리합니다.
데이터의 소유자가 반드시 메모리를 정리해야 하며 이는 중괄호가 닫히는 스코프의 끝에서 자동적으로 일어납니다.

함수를 호출 할 때의 기본 행동 양식은 메모리를 새로운 소유자로 옮기는 것입니다.
데이터를 대여만 하고 싶다면 "&"를 써서 함수가 메모리를 대여하도록 합니다.

*/
struct Book {
    pages: i32,
    rating : i32,
}

fn display_page_count(book: &Book){
    println!("pages = {:?}", book.pages);
}

fn display_rating(book : &Book){
    println!("rating = {:?}", book.rating);
}


fn main(){
    let book = Book {
        pages: 5,
        rating: 9
    };

    display_page_count(&book);
    display_rating(&book);
}