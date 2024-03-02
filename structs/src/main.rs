struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let mut user_1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("user_1@example.com"),
        sign_in_count: 1,
    };

    println!("User's email: {}", user_1.email);

    user_1.email = String::from("anotheremail@example.com");

    println!("User's email: {}", user_1.email);

    let user_2 = build_user(String::from("user_2@example.com"), String::from("username"));

    println!("User's email: {}", user_2.email);

    let user_3 = User {
        email: String::from("user_3@example.com"),
        ..user_2
    };

    println!("User's email: {}", user_3.email);

    // println!("User 2 username: {}", user_2.username); // This will fail because user_2 has been moved to user_3

    let black = Color(0, 0, 0);

    println!("Black color: {}, {}, {}", black.0, black.1, black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
