use std::f32::consts::E;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("acd@126.com"),
        username: String::from("Nikky"),
        active: true,
        sign_in_count: 556,
    };

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String) -> User {
    User {
        email,
        username: String::from("Nikky"),
        active: true,
        sign_in_count: 556,
    }
}

fn update_user(user: User) -> User {
    // User {
    //     email: String::from(""),
    //     username: String::from(""),
    //     active: user.active,
    //     sign_in_count: user.sign_in_count,
    // }
    User {
        email: String::from("newem@em.com"),
        username: String::from("newuser"),
        ..user
    }
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    let black = Color(0, 1, 2);
    print!("{}, {}, {}", black.0, black.1, black.2)
}
