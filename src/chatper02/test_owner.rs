fn main() {
    let s = String::from("hello");
    takes_ownership(&s);

    println!("{}", s);

}

fn takes_ownership(some_string : &String) {
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}