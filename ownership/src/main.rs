fn main() {

    let literal = "hello";
    println!("{literal}");
    print_type_of(&literal); // &str

    let mut s = String::from(literal);
    print_type_of(&s); // alloc::string::String

    s.push_str(", world!"); // push_str() appends a literal to a String
    // literal.push_str(", world!"); // do not work

    println!("{s}"); // this will print `hello, world!`

    let s1 = String::from("Rust"); // store data in the Heap
    let s2 = s1; // s1 becomes an ivalid reference. Rust "moved" s1 to s2

    // println!("{s1}, is cool!"); // do not work, because s1 has become an invalid reference to prevent memory corruption
    println!("{s2}, is cool!");

    let mut s = String::from("Old");
    s = String::from("New");

    println!("{s} assignment trigger Rust drop!");

    new_owner_of_s(s); // moves s to new owner
    
    // println!("{s} assignment trigger Rust drop!"); // will not work
    println!("s was dropped");

    let s2 = String::from("New New");
    let s3 = receives_and_give_back(s2); // moves s to new owner
    println!("{s3} is back. Functions returns gives back ownership");

    let (s4, len) :(String, usize) = cumbersome_length_calculator(s3);
    println!("{s4} is back, length {len} as well");

    let c1 = String::from("Clone");
    let c2 = c1.clone(); // deep copy. Copy heaps data as well

    println!("c1 = {c1}, c2 = {c2}");

    let i1 = 1; // i32 implements Copy trait
    print_type_of(&i1);

    let i2 = i1.clone(); // copy i2 to stack

    println!("i1 = {i1}, i2 = {i2}"); // works, both i1 and i2 are valid

    // now a more elegant way to calculate length without passing ownership
    // using references

    let len = calculate_length(&s4);
    println!("string: {s4}, length: {len}");

    // change_ref(&s4); //wouldn't work. references are immutable

    let mut s5 = String::from("Mut string ref");
    change_mut_ref(&mut s5);

    println!("{s5}");

    // dangling reference
    // let reference_to_nothing = dangle(); // doesn't work
    let reference_to_string = no_dangle();  // works because the ownership of string is now reference_to_string
    println!("{reference_to_string}");

  }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn new_owner_of_s(string :String) { // string is now in scope, rust moved s to string
    println!("{string} is still in scope");
} // string is now out of scope, Rust drops it

fn receives_and_give_back(string: String) -> String {  // string is now in scope, rust moved s to string
    println!("{string} was borrowed");
    string // return string, gives back ownership
}

// returning tuples is a way to pass ownership from arguments + other calculations from functions
fn cumbersome_length_calculator (string: String) -> (String, usize) {
    let len = string.len(); // len() returns the length of a String
    (string, len) // return tuple (len,string), gives back ownership
}

fn calculate_length(string_ref: &String) -> usize { // string_ref is a reference to a string
    string_ref.len()
} // string goes out of scope. Because it's not the owner of the data it refers to, it's not dropped.

// fn change_ref(string_ref :&String) {
//     string_ref.push_str(", world");
// }

fn change_mut_ref(string_ref: &mut String) {
    string_ref.push_str(" can be changed");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("string from other context");

    s
}
