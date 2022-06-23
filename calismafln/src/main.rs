#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
// unit struct
struct Unit;

// a tuple struct
struct Pair(i32, f32);

// a struct with two fileds
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) {
    // Rectangle = {bottom_right, top_left} both are points
    // destructure recursive
    let Rectangle {
        bottom_right: Point { x: x1, y: y1 },
        top_left: Point { x: x2, y: y2 },
    } = rect;

    let area = distance_between_two_points(x1, x2) * distance_between_two_points(y1, y2);
    println!("area is {}", area)
}
fn distance_between_two_points(d1: f32, d2: f32) -> f32 {
    d1.abs() + d2.abs()
}

// Enum keyword allows creation of a type which may be one of few different variants.
// any variant which is valid as a struct is also valid as an enum

// Create an enum to classify a web event. note how both names and type info
// together specify the variant

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed, '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y,)
        }
    }
}
fn main() {
    let name = String::from("Peter");

    let age = 27;
    let peter = Person { name, age };
    // print debug struct
    println!("{:?}", peter);

    // instantiate a point
    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a let bindng
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    // Make a new point by using struct update syntax to use the fileds of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    println!("{}, {}", left_edge, top_edge);

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("destructured pair contains {:?}, {:?}", integer, decimal);
    rect_area(_rectangle);

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let add_operation = Operations::Add;
    println!("{}", add_operation.run(5,2));
}

enum Operations {
    Add,
    Subtract,
}

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

