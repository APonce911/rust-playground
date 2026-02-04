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
  }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn new_owner_of_s(string :String) { // string is now in scope, rust moved s to string
    println!("{string} is still in scope");
} // string is now out of scope, Rust drops it

fn receives_and_give_back(string :String) -> String {  // string is now in scope, rust moved s to string
    println!("{string} was borrowed");
    string // return string, gives back ownership
}

// returning tuples is a way to pass ownership from arguments + other calculations from functions
fn cumbersome_length_calculator (string :String) -> (String, usize) {
    let len = string.len();     // len() returns the length of a String
    (string, len)                // return tuple (len,string), gives back ownership
}

