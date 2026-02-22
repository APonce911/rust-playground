use crate::List::{Cons, Nil};

enum List {
    // Use Box smart pointer on recursive types, 
    // Box is used to reference heap data without the compiler needing to know 
    // the exact of data, only the pointer size(udata) 
    Cons(i32, Box<List>),
    Nil,
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
}
