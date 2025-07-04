use std::{collections::HashMap, fmt::{Display}};

trait Accommodation {
  
    fn book(&mut self,name: &str,nights:u32);
}

trait Description {
    fn get_description(&self) -> String { // self is simply any type implementing Accommodation
        String::from("A wonderful place to stay")
    } 
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservation: HashMap<String,u32>
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self { name, reservation: HashMap::new()}
    }
}


impl<T: Display> Hotel<T> {
    
    fn summarize(&self) -> String {
        format!("{} {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    // fn get_description(&self) -> String {
    //     self.name.clone()
    // }

    fn book(&mut self,name: &str,nights:u32) {
        // self.name = name.to_string();
        self.reservation.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {
    
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

fn mix_and_match_with_generics_fix_with_multiple_trait_bounds_with_where<T, U> (first: &mut T, second: &mut U, guest: &str) where  T: Accommodation + Description, U: Accommodation{ // here both the types T are the same meaning they will be only one type
    first.book(guest, 1);
    second.book(guest, 1);
}


fn choose_best_place_to_stay() -> impl Accommodation + Description {
    let luxury = true;
    // here the type that we return mater too and should be consistent meaning if Hotel is being returned in an if clause we need to make sure to return the Hotel in else class too
    if luxury {
        
        Hotel::new("The Luxe")
    }else {
        // AirBnB::new("Peter")   // This will throw an error for sure
        Hotel::new("The Non Luxe") // added this to satisfy the return value comment this and un comment above to see the error
    }

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
    

    // This is important and will give you a good understanding of rust type system
    let mut hotel3 = choose_best_place_to_stay(); // observe this the type is impl Accommodation + Description and Hotel type
    // hotel3.summarize(); // This will give an error because the type of hotel3 is not Hotel.

    let h1 = Hotel::new(String::from("kye")); 
    let h2 = Hotel::new("meowww"); 
    let h3 = Hotel::new(    vec![1,2,3]); 

    println!("{}",h1.summarize());
    println!("{}",h2.summarize());
    // println!("{}",h3.summarize()); // This will throw an error because a vector doesn't implement Display 


    // dynamic dispatch is for runtime 
    // dynamic dispatch will figure out the type in runtime
    // dynamic dispatch will only work on references
    // A trait object is some instance of some type that implements a particular trait
    let stays: Vec<&dyn Description> = vec![&h1,&airBnB2];

    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());



}