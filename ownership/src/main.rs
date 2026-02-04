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
    print!("{string} is still in scope");
} // string is now out of scope, Rust drops it
