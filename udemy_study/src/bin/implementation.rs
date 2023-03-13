/*
impl 키워드는 어떤 열거형이나 구조체에 기능을 구현하게 하며 이를 통해 코드의 구성을 더 좋게 만들고 플그램을 더 읽기 쉽게 만듭니다.

*/

struct Temperature {
    degrees_f : f64
}


// impl 키워드 뒤에 구조체나 열거형의 이름을 씁니다.
impl Temperature {
    // 대문자 Self는 아직 안만들어지지 않아 새로만드는 경우나 Temperature를 참조할때 사용합니다. 
    fn freezing() -> Self {
        Self {
            degrees_f : 32.0
        }
    }

    fn boiling() -> Self {
        Self { degrees_f : 212.0}
    }


    //fn show_temp(temp: Temperature){
    fn show_temp(&self){
        println!("{:?} degrees F", self.degrees_f);
    }
}

fn main(){
    let hot = Temperature {degrees_f: 99.9};
    //Temperature::show_temp(hot);
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
}