fn main() {
    let number = 5;

    // Unlike ruby, it does not autoconvert other types to bool
    if number == 5 {
        println!("{} equal to five", number);
    } else if number < 5 {
        println!("{} is less than five", number);
    } else {
        println!("{} more than five", number);
    }

    let condition = true;

    // inline
    let number = if condition { 5 } else { 6 };

    println!("number: {number}")
}
