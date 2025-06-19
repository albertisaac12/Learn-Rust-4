use std::{collections::HashMap};
use std::fmt::{format, Display};
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

fn book_for_one_night (entity: &mut impl Accommodation)  {
    println!("{}", entity.get_description());
    entity.book("new Guest", 1);
}

fn book_for_one_night_trait_bound<T: Accommodation> (entity: &mut T) {
    println!("{}", entity.get_description());
    entity.book("nnnn", 4);
}

fn mix_and_match(first: &mut impl Accommodation, second: &mut impl Accommodation,guest: &str) { // here first and second can be any types as long as they implement the Accommodation trait
    first.book(guest, 1);
    second.book(guest, 1);
}

fn mix_and_match_2<T: Accommodation>(first: &mut T, second: &mut T,guest: &str) { // here both first and second are of both type T
    first.book(guest, 1);
    second.book(guest, 1);
}

fn mix_and_match_2_fix<T: Accommodation,U: Accommodation>(first: &mut T, second: &mut U,guest: &str) { // here both first and second are of both are different types Now
    first.book(guest, 1);
    second.book(guest, 1);
}

fn nxx<T,U> (first: &mut T,second: &mut U) where T: Accommodation, U: Accommodation {

}

fn traits_as_return_types()  -> impl Accommodation { // Any type that implements Accommodation
   Hotel::new("mwo mwo")
}

// fn in_traits_as_return_types_we_cannot_return_differnt_types() -> impl Accommodation { // this function will give types mismatched error

//     if true {
//        return Hotel::new("aaaa");
//     }else {
//         return AirBnB::new("asdasd");
//     }

// }

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

    book_for_one_night(&mut h1);


    let stays: Vec<&dyn Accommodation> = vec![&h1,&a1];

}

// if a default implementation is not overridden it will not be able to call by instance of that type rather use the type and :: . Example ->  Type::function();

// first : &mut (impl trait1 + trait2)

struct mc<T> /*mc<T: Accommodation> */ { // Any type T that implements Accommodation
    name : T,
    reservations : HashMap<String,u32>
}

impl<T: Display> mc<T> {
    fn summarize(&self) -> String {
        format!("{} {}",self.name,self.name)
    }
}

impl<T>  mc<T> {
  fn new(name:T) -> Self {
    Self { name, reservations: HashMap::new() }
  }
}