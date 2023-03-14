/*
열거형 및 구조체에 기능을 직접 구현하지 않고 추가하는 방법에 대해 알아봅니다.
파생(derive) 매크로를 사용하면 됩니다.
*/


// derive는 열거형 및 구조체에 쓸 수 있는 특수 매크로입니다.
// Debug를 추가하면 아래 match 함수 없이 쓸 수 있습니다.
// Clone이나 Copy를 사용하면 구조체에 저장하거나 함수를 넘길 때 사본을 만들어 넘기도록 지시합니다.
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

// Position에 Clone과 Copy가 있기 때문에 Employee도 추가해줘야합니다.
#[derive(Debug, Clone, Copy)]
struct Employee {
    position : Position,
    work_hours : i64
}

fn print_employee(emp: Employee){
    println!("{:?}", emp);
}


fn main(){
    let me = Employee{
        position : Position::Worker,
        work_hours : 40
    };
    // match me.position {
    //     Position::Manager => println!("manager"),
    //     Position::Supervisor => println!("supervisor"),
    //     Position::Worker => println!("worker")
    // }

    println!("{:?}", me.position);
    println!("{:?}", me);
    print_employee(me)
}