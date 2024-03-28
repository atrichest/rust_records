//A User struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//A build_user function that takes an email and username and returns a User instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//A build_user function that uses field init shorthand
//because the username and email parameters have the same name as struct fields
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

//Ownership of Struct Data
//we used the &str string slice type rather than the owned String type
struct User_Lifetime<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

fn main() {
    //Creating an instance of the User struct
    let user1 = User {
        active: true,
        username: String::from("aiclr"),
        email: String::from("aiclr@qq.com"),
        sign_in_count: 1,
    };

    //Changing the value in the email field of a User instance
    let mut user1 = User {
        active: true,
        username: String::from("aiclr"),
        email: String::from("aiclr@qq.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("xxx@qq.com");

    let user1 = build_user(String::from("aiclr"), String::from("aiclr@qq.com"));

    //Creating a new User instance using one of the values from user1
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("xxx@qq.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user1 = build_user_shorthand(String::from("aiclr"), String::from("aiclr@qq.com"));
    //Using struct update syntax to set a new email value for a User instance
    //but to use the rest of the values from user1
    let user2 = User {
        email: String::from("aiclr@example.com"),
        //The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
        ..user1
    };

    //struct with lifetime
    let user1 = User_Lifetime {
        active: true,
        username: "aiclr",
        email: "aiclr@qq.com",
        sign_in_count: 1,
    };
}
