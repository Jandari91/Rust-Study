struct Hostname(String);

fn connect(host: Hostname){
    println!("connected to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    connect(host);
}


// use std::rc::Rc;
// use std::sync::{Arc, Mutex};

// fn main() {
//     let a = 10;
//     let b = Box::new(20);
//     let c = Rc::new(Box::new(30));
//     let d = Arc::new(Mutex::new(40));
//     println!("a:{:?}, b : {:?}, c:{:?}, d:{:?}", a,b,c,d)


    
//     //test_equals();
// }


// fn test_equals(){
//     let a = 10;
//     // 비교(==)를 써야하는 곳에 대입(=)을 하면 컴파일 에러가 난다.
//     if a == 10 {
//         println!("a equals ten");
//     }
// }