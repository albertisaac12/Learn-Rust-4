use std::{collections::HashMap, fmt::{format, Debug}};

trait Accommodation {
  
    fn book(&mut self,name: &str,nights:u32);
}

trait Description {
    fn get_description(&self) -> String { // self is simply any type implementing Accommodation
        String::from("A wonderful place to stay")
    } 
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservation: HashMap<String,u32>
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self { name: name.to_string(), reservation: HashMap::new()}
    }

    fn summarize(&self) -> String {
        format!("{} {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    // fn get_description(&self) -> String {
    //     self.name.clone()
    // }

    fn book(&mut self,name: &str,nights:u32) {
        // self.name = name.to_string();
        self.reservation.insert(name.to_string(), nights);
    }
}

impl Description for Hotel {
    
}

#[derive(Debug)]
struct AirBnB {
    host : String,
    guests: Vec<(String,u32)>
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self { host: host.to_string(), guests: vec![] }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self,name: &str,nights:u32) {
        self.guests.push((name.to_string(),nights));
    }

    
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment ", self.host)
    }
}

fn book_for_one_night<T:Accommodation>(entity: &mut T,guest: &str) {
    // println!("{}", entity.get_description());
    entity.book(guest, 1);
}

fn mix_and_match (first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

fn mix_and_match_with_generics<T: Accommodation> (first: &mut T, second: &mut T, guest: &str) { // here both the types T are the same meaning they will be only one type
    first.book(guest, 1);
    second.book(guest, 1);
}

fn mix_and_match_with_generics_fix<T: Accommodation, U: Accommodation> (first: &mut T, second: &mut U, guest: &str) { // here both the types T are the same meaning they will be only one type
    first.book(guest, 1);
    second.book(guest, 1);
}

fn mix_and_match_with_generics_fix_with_multiple_trait_bounds<T: Accommodation + Description, U: Accommodation> (first: &mut T, second: &mut U, guest: &str) { // here both the types T are the same meaning they will be only one type
    first.book(guest, 1);
    second.book(guest, 1);
}
fn main() {

    let mut booking1 = Hotel::new("Jolly Stayz");
    println!("{}",booking1.get_description());
    booking1.book("meow", 5);
    println!("{:?}", booking1);
    println!("{}",booking1.summarize());
    book_for_one_night(&mut booking1, "wowzaaa");
    
    let mut airBnB = AirBnB::new("Peter");
    println!("{}", airBnB.get_description());
    airBnB.book("meow", 2);
    println!("{:?}", airBnB);
    book_for_one_night(&mut airBnB, "billa`");
    
    
    let mut booking2 = Hotel::new("Jolly Stayz");
    let mut airBnB2 = AirBnB::new("Peter");
    mix_and_match(&mut booking2, &mut airBnB2, "meowwww");
    mix_and_match(&mut airBnB2, &mut booking2, "meowwww");
    // mix_and_match_with_generics(&mut airBnB2, &mut booking2, "meowwww"); // This wil give an error because booking2 and airBnB2 are different types even if they implement the Accommodation trait
    mix_and_match_with_generics_fix(&mut airBnB2, &mut booking2, "meowwww");
    
}