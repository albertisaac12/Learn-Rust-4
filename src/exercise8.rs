/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.
 
Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.
 
Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the `!` symbol and return a vector of the string
slices. Invoke the function in `main`.
 
Example:
elements("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]
 
Define a `get_identity` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the `expect`
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `get_identity` function in `main`, and output the
returned String.
 
Example:
fn main() {
  let name = get_identity();
   println!("{name}"); // Bill Murray
}
*/

use std::io;
fn main() {
    let mut s1 = String::from("1000");
    make_money(&mut s1);
    println!("{s1}");
    
    let mut s1 = String::from("avx   ");
    trim_and_capitalize(&mut s1);
    println!("{s1}");

    let vc = elements("Gold!Silver!Platinum");

    println!("{:?}",vc);

    let name = get_identity();
    println!("{name}");
    


}

fn make_money(ree: &mut String) {
    ree.push_str("$$$");
}

fn trim_and_capitalize(ree: &mut String)-> String {
    ree.trim().to_uppercase()
}

fn elements(ree:&str)-> Vec<&str> {
    ree.trim().split("!").collect::<Vec<&str>>()
}

fn get_identity()-> String {
    println!("Enter Your First Name: ");
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).expect("Failed To Read First Name");
    let mut s2 = String::new();
    println!("Enter Your Last Name: ");
    io::stdin().read_line(&mut s2).expect("Failed To Read Last Name");
    s1 = s1.trim().to_string();
    s2 = s2.trim().to_string();
    s1 + " " + &s2
}