#![allow(dead_code)]
use crate::List::*;

fn main() {
    // use_example();
    // c_like_enum();
    // linked_list_impl();
    // const_ex()
    // variable_bindngs();
    // mutability()
    scope()
    
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

fn const_ex() {
    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;
    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The treshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}

fn variable_bindngs() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    let copied_integer = an_integer;
    println!("An Integer : {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
}

fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    //Ok
    mutable_binding += 1;
    println!("After Mutation: {}", mutable_binding);
}

fn scope() {
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    //Error! `short_lived_binding` does not exist in this scope
    //println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    let shadowed_bindng = 1;
{
        println!("before being shadowed: {}", shadowed_bindng);
        let shadowed_bindng = "abd";
        println!("shadowed in inner block : {}", shadowed_bindng);
    }
    println!("outside inner block: {}", shadowed_bindng);

    let shadowed_bindng = 2;
    println!("shadowed in outer block : {}", shadowed_bindng);
}
