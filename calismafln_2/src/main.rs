#![allow(dead_code)]
use crate::List::*;

fn main() {
    // use_example();
    // c_like_enum();
    linked_list_impl();
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}
fn use_example() {
    // Explicitly use each name so the are awailable without manual scoping
    use crate::Status::{Poor, Rich};
    //Automatically use each name inside Work
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money ..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

enum Number {
    Zero,
    One,
    Two,
}
enum Color {
    Red = 0xff000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn c_like_enum() {
    println!("Zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("Violets are #{:06x}", Color::Blue as i32)
}

enum List {
    // Cons:: Tuple struct that wraps an element and a pointer to next node
    Cons(u32, Box<List>),
    // end of linked list
    Nil,
}
// methods can be attached to an enum
impl List {
    // create an empty list
    fn new() -> List {
        Nil
    }
    // Consume a list, and return the same list with a new element
    fn prepend(self, elem: u32) -> List {
        // cons also has type list
        Cons(elem, Box::new(self))
    }
    // return length of list
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
}

fn linked_list_impl() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length : {}", list.len());
    println!("{}", list.stringify());
}
