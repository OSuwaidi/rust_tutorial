// بسم الله الرحمن الرحيم وبه نستعين

use std::f32::consts::PI;

// "enum" is some custom *datatype* that can be used as an argument or returned from a function/method
enum Shape {
    Circle(f32),         // represents "radius" argument
    Rectangle(f32, f32), // represents "width" and "height" arguments
    Square(f32),         // represents "side" argument
}

fn area(shape: &Shape) -> f32 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);
    let square = Shape::Square(3.0);

    println!("Area of the circle: {:.2}", area(&circle));
    println!("Area of the rectangle: {:.2}", area(&rectangle));
    println!("Area of the square: {:.2}", area(&square));
}
