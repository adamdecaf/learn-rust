fn add_one(x: i32) -> i32 {
    x + 1
}

// Probably shouldn't call this..
fn diverges() -> ! {
    panic!("Woah bro, I'm crashing")
}

struct Point {
    x: i32,
    y: i32,
    z: i32
}

// struct Point2(i32, i32, i32);

enum Color {
    Red,
    // Green,
    // Blue,
    // Other,
}

fn main() {
    println!("Hello World!");

    let x: i32 = 1;
    let (y, z) = (2,3);

    println!("The value of (x, y, z) is ({}, {}, {})", x, y, z);

    if x == 0 {
        println!("the x coordinate is zero!");
        diverges();
    }

    println!("x + 1 = {}", add_one(x));

    let p1 = Point { x: 2, y: 3, z: 4 };
    println!("p1.x, p1.y, p1.z = {}, {}, {}", p1.x, p1.y, p1.z);

    // let p2 = Point2(3,4,5);
    // println!("p2 = {:?}", p2);

    let c = Color::Red;
    match c {
        // Color::Green => println!("c is green"),
        // Color::Blue  => println!("c is blue"),
        Color::Red  => println!("c is red"),
        // Color::Other  => println!("c is other")
    }
}
