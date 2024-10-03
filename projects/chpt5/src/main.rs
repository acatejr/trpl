fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // println!("User email {}", user1.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}