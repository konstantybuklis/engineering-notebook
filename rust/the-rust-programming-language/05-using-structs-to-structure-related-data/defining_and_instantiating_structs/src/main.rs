struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!(
        "user1: {}, {}, {}, {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    user1.email = String::from("anotheremail@example.com");
    println!("user1 new email: {}", user1.email);

    let user2 = build_user(
        String::from("newuser123"),
        String::from("newuser@example.com"),
    );

    // struct update syntax
    let _user3 = User {
        email: String::from("anotheremail@example.com"),
        ..user2
    };

    // won't work because username from the user2 was moved to the user3 instance
    // println!("{}", user2.username);

    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("x = {x}, y = {y}, z = {z}");

    let black = Color(0, 0, 0);
    let Color(r, g, b) = black;
    println!("r = {r}, g = {g}, b = {b}");

    let _subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1,
    }
}
