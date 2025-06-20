// trait bound syntax on methods
use std::{collections::HashMap, fmt::{format, Display}};

use std::ops::Add;

trait Accommodation {
      
    fn book(&mut self, name: &str,nights: u32) -> ();

    // default implementation 
    fn default() {
        println!("This is a default Implementation");
    }
}



trait Description {
    fn get_description(&self) -> String {
        "The best Hotel ever".to_string()
    }
}


#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String,u32>
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self { name, reservations: HashMap::new() }
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str,nights: u32) -> () {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name,self.get_description())
    }
}


#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String,u32)>
}

impl AirBnB {
    fn new(host: &str) -> Self{
        Self { host: host.to_string(), guests: vec!() }
    } 
}


impl Accommodation for AirBnB {

    fn book(&mut self, name: &str,nights: u32) -> () {
        self.guests.push((name.to_string(),nights));
    }
}


impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("The best host in the world {}",self.host)
    }
}

fn main() {
 
    let h1 = Hotel::new("jooo".to_string());

    println!("{}",h1.summarize());

    let h2 = Hotel::new(vec!["aa".to_string()]);
    println!("{}",h2.get_description());
    
    let hotel = Hotel::new(String::from("The Luxe"));
    let airbnb = AirBnB::new("Peter");

    let stays: Vec<&dyn Description> = vec![&hotel,&airbnb];

    println!("blahhhh");
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());


}


// A trait object is an instance of a type that implements a particular trait whose methods will be accessed at runtime using a feature called dynamic dispatch

// Trait must be in scope to Use its Definitions

// An associated constant is a constant declared within the trait A constant is a name for a fixed immutable value