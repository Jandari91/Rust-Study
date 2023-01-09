use std::convert::TryInto;

fn main() {
    let a = 10;
    let b: u16 = 100;

    let bb = b.try_into()
            .unwrap();

    if a < bb {
        println!("a is less than b");
    }   
}