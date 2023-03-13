enum Color {
    Blue, Red
}
impl Color {
    fn print(&self){
        match self {
            Color::Blue => println!("blue"),
            Color::Red => println!("red"),
        }
    }
}
struct Dimensions{
    width : f64,
    height : f64,
    depth : f64
}

impl Dimensions {
    fn print(&self) {
        println!("width : {:?}", self.width);
        println!("height : {:?}", self.height);
        println!("depth : {:?}", self.depth);
    }
}

struct ShippingBox {
    weight : f64,
    color : Color,
    dimensions : Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color:Color, dimensions:Dimensions) -> Self{
        Self{
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight : {:?}", self.weight);
    }
}

fn main(){
    let small_dimensions = Dimensions {
        width : 1.0,
        height : 2.0,
        depth : 3.0
    };


    let shippingBox = ShippingBox::new(5.0, Color::Red, small_dimensions);
    shippingBox.print();
}