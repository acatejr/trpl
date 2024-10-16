fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };

    println!("User email {}", user1.email);
    println!("User email {}", user2.email);
    println!("User email {}", user3.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}