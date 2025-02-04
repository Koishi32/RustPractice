struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123".to_string(),
        email: "someone@example.com".to_string(),
        sign_in_count: 1,
    };
}
