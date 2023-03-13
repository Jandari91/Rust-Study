/*
Rust는 표현식에 기반한 언어로, 곧 대부분의 것들이 평가되어 값을 반환한다는 의미입니다.
이러한 표현식의 값들은 하나의 점으로 병합되므로 중첩로직에 사용할 수 있습니다.

let is_it_5 = my_num > 5;

위와 같이 중첩 로직을 사용 할 수 있습니다.

표현식을 쓰면 if와 match을 중첩해 중첩 로직을 사용 할 수 있게 되지만 두,세 단계 이상으로 중첩하게 되면 코드가 굉장히 난해해집니다.


*/

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}



fn main(){
    // secret file : admins only
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    println!("access : {:?}", can_access_file);
}