use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String; // self is simply any type implementing Accommodation
    fn book(&mut self,name: &str,nights:u32);
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
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        self.name.clone()
    }

    fn book(&mut self,name: &str,nights:u32) {
        // self.name = name.to_string();
        self.reservation.insert(name.to_string(), nights);
    }
}



fn main() {

    let mut booking1 = Hotel::new("Jolly Stayz");
    println!("{}",booking1.get_description());
    booking1.book("meow", 5);
    println!("{:?}", booking1);


}