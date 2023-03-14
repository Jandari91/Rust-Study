
enum Color {
    Blue, Red, Gray
}

struct Person {
    age : i32,
    name : String,
    fav_color : Color
}

fn print(data: &str){
    println!("{:?}", data);
}


fn main(){
    let people = vec![
        Person {
            age: 8,
            name : String::from("Georage"),
            fav_color : Color::Red
        },
        Person {
            age: 22,
            name : String::from("bar"),
            fav_color : Color::Blue
        },
    ];

    for person in people{
        if person.age <= 10 {
            print(&person.name);
        }
    }
}