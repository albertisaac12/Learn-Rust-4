// #![allow(unused_variables)] // compiler directive for the entire code to allow unused variables
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point(i32, i32, i32); // Tuple Structs are use full when one wants to give an entire tuple a name

struct unit_struct; // unit struct is used when one wants to implement a trait on some type but don't want any data.git

struct rectangle {
    height: u64,
    width: u64,
}
fn main() {
    let mut user1 = buildUser("blah@gmail.com".to_string(), String::from("blah"));

    user1.email = "meow@gmail.com".to_string();

    // println!("{:#?}",user1);

    // building a user from another user

    let user2 = User {
        email: "meow2@gmail.com".to_string(),
        ..user1
    };

    println!("{:?}", user1.sign_in_count);
    println!("{:?}", user1.active);

    // the above two prints work completely fine even after the print happen after user1 is moved into user2 reason is both bool and u64 are copy types
    // the below will give an error because the String are not a copy type in any way or manner

    // println!("{:?}",user1.username); // here the String DOES NOT IMPLEMENT THE COPY TYPE HENCE THE ERROR

    let p1 = Point(1, 1, 1);

    println!(
        "{:?}",
        area(&rectangle {
            height: 50,
            width: 10
        })
    );

    let r1 = rectangle {
        height: 10,
        width: 50,
    };
    println!("{}", r1.area());


    let r2 = rectangle::square(5);

    let r3 =5;

}

fn buildUser(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

fn area(rect: &rectangle) -> u64 {
    rect.height * rect.width
}

impl rectangle {
    fn area(&self) -> u64 {
        // self represents the instance of struct the method is being called on.
        self.height * self.width
    }

    fn square(side: u64) -> Self {
        rectangle {
            height: side,
            width: side,
        }
    }
}