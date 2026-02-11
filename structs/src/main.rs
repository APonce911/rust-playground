// we're using owned types here like String
// it's possible tu use references like &str as well using Lifetimes (cap10)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    favorite_color: Color
}

// include debugging functionallity to Color struct using Debug trait
// derive attribute
// https://doc.rust-lang.org/reference/attributes/derive.html
#[derive(Debug)]
struct Color(i32, i32, i32); // tuple like struct

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        favorite_color: Color(255, 255, 255)
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
    let mut user3 = User {
      email: String::from("email3@example.com"),
      ..user2
    };

    println!("{}", user3.email);
    println!("{}", user2.email);
    // println!("{}", user2.username); 
    // do not work because field was moved to user3 when we used struct update syntax
    // the username field type (String) doesn't implement the copy trait

    println!("{}, {}, {}", user1.favorite_color.0, user1.favorite_color.1, user1.favorite_color.2);

    // destructuring
    let Color(r, g, b) =   user1.favorite_color;
    println!("{}, {}, {}", r, g, b);

    let orange = Color(255, 128, 0);
    user3.favorite_color = orange;

    println!("{}, {}, {}", user3.favorite_color.0, user3.favorite_color.1, user3.favorite_color.2);
    // println!("{}, {}, {}", user2.favorite_color.0, user2.favorite_color.1, user2.favorite_color.2);
    // same as username, field value moved to user3

    // Color doesn't implement std::fmt::Display
    // so we use the specifier :? to use output format called Debug
    println!("{:?}", user3.favorite_color);

    // {:#?} to pretty print
    println!("user3 favorite color is {:#?}", user3.favorite_color);

    // dbg! macro. It uses stderr. It takes the ownership and return. Uses & to reference
    // dbg!(user3.favorite_color);
    // println!("user3 favorite color is {:#?}", user3.favorite_color); would not work if moved to dbg
    dbg!(&user3.favorite_color);

    // works because main still owns
    println!("user3 favorite color is {:#?}", user3.favorite_color);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        favorite_color: Color(0, 0, 0)
    }
}
