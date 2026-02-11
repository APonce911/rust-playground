struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotherusername123@example.com");
    println!("{}", user1.email);

    let user2 = build_user("user2email@example.com".to_string(), "user2username".to_string());
    println!("{}", user2.email);

    // struct update syntax
    // same as
    // let user3 = User {
    //     active: user2.active,
    //     username: user2.username,
    //     email: String::from("email3@example.com"),
    //     sign_in_count: user2.sign_in_count,
    // }; 
    let user3 = User {
      email: String::from("email3@example.com"),
      ..user2
    };

    println!("{}", user3.email);
    println!("{}", user2.email);
    // println!("{}", user2.username); 
    // do not work because field was moved to user3 when we used struct update syntax
    // the username field type (String) doesn't implement the copy trait
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
