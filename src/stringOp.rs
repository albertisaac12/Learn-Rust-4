use std::io;

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

    println!("{}",music.replace("a", "@"));
    // println!("{music}");

    let genres= music.split(", ").collect::<Vec<&str>>();

    let mut line = String::new();
    // let mc =io::stdin().read_line(&mut line).expect("Error Reading from Console"); // Ok() here has the number of bytes
    // println!("{mc}");
    match io::stdin().read_line(&mut line) {
        Ok(a) => {
            println!("{a}");
        }
        _=> {
            println!("Error Reading");
        }
    } // Ok() here has the number of bytes
    println!("{line}");

}