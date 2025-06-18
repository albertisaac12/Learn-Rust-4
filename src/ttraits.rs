use std::{collections::HashMap};

trait Accommodation {
    fn get_description(&self) -> String;
    
    fn book(&mut self, name: &str,nights: u32) -> ();

    // default implementation 
    fn default() {
        println!("This is a default Implementation");
    }
}

#[derive(Debug)]
struct Hotel {
    name:String,
    reservations: HashMap<String,u32>
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() , reservations: HashMap::new() }
    }
}

impl Accommodation for Hotel {

    fn get_description(&self) -> String {
       format!("{} is the pinnacle of luxury",self.name)
    }
    fn book(&mut self, name: &str,nights: u32) -> () {
        self.name = name.to_string();
        self.reservations.insert(name.to_string(), nights);
    }

    fn default() {
        println!("Overriding the Default implementation in Hotel")
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
    fn get_description(&self) -> String {
        format!("The best host in the world {}",self.host)
    }

    fn book(&mut self, name: &str,nights: u32) -> () {
        // self.host = name.to_string();
        self.guests.push((name.to_string(),nights));
    }
}

fn book_for_one_night (entity: &impl Accommodation)  {
    println!("{}", entity.get_description());
}

fn main() {

    let mut h1 = Hotel::new("wowzzzaaaa");

    h1.get_description();
    h1.book("lukla", 5);

    let mut a1 = AirBnB::new("mira");
    a1.get_description();
    a1.book("myro", 4);

    println!("The Hotel is : {:?},\nThe AirBnb is {:?}",h1,a1);

    Hotel::default();
    AirBnB::default();

    book_for_one_night(&h1);

}

// if a default implementation is not overridden it will not be able to call by instance of that type rather use the type and :: . Example ->  Type::function();