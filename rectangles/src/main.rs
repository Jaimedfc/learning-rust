fn main() {
    let width1 = 30;
    let height1 = 50;

    let rec = (30, 50);

    let rec_struct = Rectangle {
        width: 30,
        height: 50
    };
    let rec_struct_2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rec_struct_3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle defined by 2 variables is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle defined by a tuple is {} square pixels.",
        area_of_tuple(rec)
    );

    println!(
        "The area of the rectangle defined by a struct is {} square pixels.",
        area_of_struct(&rec_struct)
    );

    println!(
        "The area of the rectangle defined by a struct and calculated by a method is {} square pixels.",
        rec_struct.area()
    );

    println!("Can rec_struct hold rec_struct_2? {}", rec_struct.can_hold(&rec_struct_2));
    println!("Can rec_struct hold rec_struct_3? {}", rec_struct.can_hold(&rec_struct_3));

    println!("This is my Rectangle: {rec_struct:?}");

    let sqr = Rectangle::square(42);
    println!("This is a square: {sqr:#?}")
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_of_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area_of_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}