fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("Some Name"),
        sign_in_count: 1,
        active: true,
    };

    let mut user2 = User {
        email: String::from("email@example.com"),
        username: String::from("Pepe"),
        sign_in_count: 1,
        active: true,
    };
    user2.email = String::from("new_email@example.com");
    user2.username = String::from("Changed");

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            sign_in_count: 1,
            active: true,
        }
    }

    fn build_user_again(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let _user3 = build_user(String::from("another@example.com"), String::from("Alfonso"));
    let _user4 = build_user_again(String::from("email@server.com"), String::from("Maria"));

    let _user6 = User {
        email: String::from("hello@example.com"),
        username: String::from("Xoan"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let _user7 = User {
        email: String::from("seven@example.com"),
        username: String::from("Seven"),
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("This is a colour: #{:02}{:02}{:02}", black.0, black.1, black.2);
    println!("This is a point: ({}, {}, {})", origin.0, origin.1, origin.2);
}
