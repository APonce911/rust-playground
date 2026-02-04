const TRANSVERSAL_CONSTANT :u32  = 400;

// Static work as global variable.
// fixed address in memory
static MY_NAME : &str = "Airton";

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    let y = 10;
    println!("The value of y is: {y}");

    if true {
        let y = 11;
        println!("The value of y is: {y}");
    }

    if true {
        let y: f32 = 1.1;
        println!("The value of y is: {y}");
    }

    x = 6;
    println!("The value of x is: {x}");

    println!("The value of TRANSVERSAL_CONSTANT is: {TRANSVERSAL_CONSTANT}");
    println!("{}", TRANSVERSAL_CONSTANT);
    println!("{}", MY_NAME);
}
