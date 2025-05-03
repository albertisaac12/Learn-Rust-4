#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email : String,
    sign_in_count:u64,
}


struct Point(i32,i32,i32); // Tuple Structs are use full when one wants to give an entire tuple a name

struct unit_struct; // unit struct is used when one wants to implement a trait on some type but don't want any data.git

fn main() {

    let mut user1 = buildUser("blah@gmail.com".to_string(), String::from("blah"));

    user1.email = "meow@gmail.com".to_string();

    // println!("{:#?}",user1);

    // building a user from another user

    let user2 = User {
        email: "meow2@gmail.com".to_string(),
        ..user1
    };

    println!("{:?}",user1.sign_in_count);
    println!("{:?}",user1.active);
    
    // the above two prints work completely fine even after the print happen after user1 is moved into user2 reason is both bool and u64 are copy types
    // the below will give an error because the String are not a copy type in any way or manner

    // println!("{:?}",user1.username); // here the String DOES NOT IMPLEMENT THE COPY TYPE HENCE THE ERROR


    
    let p1 = Point (1,1,1);

}

fn buildUser(email:String,username:String) -> User {

    User {
        active:true,
        email,
        username,
        sign_in_count:1,
    }


}