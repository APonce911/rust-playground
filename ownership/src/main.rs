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
  }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
