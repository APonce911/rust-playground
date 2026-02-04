fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break returns value 20 as a statement(w ;) or as a expression(w/o ;) 
            break counter * 2
        }
    };

    println!("The result is {result}");
    counter_loop();
    while_countdown(result);

    // changes to result variables in the while countdown context did not affect result immutable variable
    // it creates a copy of the value of the variable result and assign number
    println!("The result is {result}");
}

fn counter_loop() {
  let mut outer_counter = 0;
  let mut inner_counter;

  'outer_loop: loop {
    outer_counter += 1;
    inner_counter = 0;

    loop { 
      if outer_counter + inner_counter == 20 { break 'outer_loop };

      if inner_counter == 9 { break };
      inner_counter+=1;
    };
  };

  println!("inner_counter: {}, outer_counter: {}", inner_counter, outer_counter);
}

// param number must be mut so it could assign new values in this scope
fn while_countdown(mut number :u32) {
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}
