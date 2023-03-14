fn main(){
    let my_number = vec![10, 20, 30, 40];
    for number in &my_number { // & 안 붙이면 소유권 이동으로 컴파일 되지 않음
        match number {
            30 => println!("number = thirty"),
            _ => println!("number = {:?}", number)
        };
    }

    println!("number of elements = {:?}", my_number.len());
}