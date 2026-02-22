use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    // Use Box smart pointer on recursive types, 
    // Box is used to reference heap data without the compiler needing to know 
    // the exact of data, only the pointer size(udata) 
    Cons(i32, Box<List>),
    Nil,
}


// tuple type with single element T
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // associated type(https://doc.rust-lang.org/book/ch20-03-advanced-types.html#type-synonyms-and-type-aliases)
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // reference inner data - access MyBox tuple at index 0
        &self.0
    }
}

struct BoxWithoutDeref<T>(T);

impl<T> BoxWithoutDeref<T> {
    fn new(x: T) -> BoxWithoutDeref<T> {
        BoxWithoutDeref(x)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let z = &x;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *z);
    // deref operator works the same for boxes and references(&)
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));

    // deref coercion calls deref recursively to reach desided type(&str)
    hello(&m);
    //otherwise, without deref coecion
    hello(&(*m)[..]);
    // or
    hello(&(*m));
    // or
    hello(&*m);
    // or without defer trait
    let m = BoxWithoutDeref::new(String::from("without defer trait"));
    hello(&m.0);

    // do not work
    // hello(&m);
    drop_test();
    println!("End of program")
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn drop_test() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created");

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
    // std::mem::drop
    drop(c);
    println!("CustomSmartPointers created");
} // without calling drop(), Rust drops automatically in LIFO order when OOC
 
