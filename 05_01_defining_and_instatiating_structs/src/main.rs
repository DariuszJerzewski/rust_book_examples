struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user = User {
        active: true,
        username: String::from("zryw"),
        email: String::from("zryw@zryw.zryw"),
        sign_in_count: 0,
    };

    user.sign_in_count = 1;

    let user = create_user("test", "email");
    let user = User {
        username: String::from("new"),
        ..user
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;

}

fn create_user(username: &str, email: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 0,
    }
}
