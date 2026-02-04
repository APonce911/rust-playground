fn main() {
    let x = five();

    println!("The value of x is: {x}");
    print_labeled_measurement(five(), 'h');
    print_labeled_measurement(plus_one(x), 'h');
}

// snake_case definition
fn five() -> i32 {
    5
}

// return type omitted, return unit ()
// NOTE: No named parametes. Can use strucs with default value or builder pattern
fn print_labeled_measurement(value: i32, unit_label: char) {  
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
