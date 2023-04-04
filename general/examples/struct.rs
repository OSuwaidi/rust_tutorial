// بسم الله الرحمن الرحيم وبه نستعين

fn main() {
    println!("Creating a 2D point...");
    let mut p = Point::new(3., 4.);
    println!("Distance from origin is: {}", p.dist_from_origin());
    p.x = 10.; // must be of same datatype as struct "Point" accepts
    p.y = 11.;
    println!("New distance is: {:.2}", p.dist_from_origin());
}

// "struct" is similar to a "class", but its method are defined outside of its definition
// "struct"s also do not have constructors, you create instances by using an associated method
struct Point {
    // "struct"'s attributes/arguments:
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        // you can either return "Self" or the struct "Point"
        Point { x, y } // if you want, you can add "return ___ ;", but rust has implicit returns without ";"
    }

    fn dist_from_origin(&self) -> f32 {
        // "&self" being a reference to "Point"
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
