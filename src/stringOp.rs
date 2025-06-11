use std::io::{read_to_string};

fn main() {
    
    let mut name = String::from("meow");

    name.push_str("gg ");

    name.push('w');


    let f_name = String::from("meow");
    let l_name = String::from("ggg");

    let c_name = f_name + &l_name;

    println!("{}",&c_name);

    let icon = format!("hello {} ", l_name);

    println!("{icon}");
    println!("{l_name}");
    
    // icon.split("hello");

    icon.trim();
    
    println!("{icon}");

    let music = "Rock, Metal, Country, Rap";

    let genres= music.split(", ").collect::<Vec<&str>>();

}