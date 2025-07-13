use std::fmt::{format, Debug, Display, Formatter};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) -> String {
        self.get_data()
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32
}

impl<T> Coffee<T> {
    fn new(kind:T,milk:Milk,ounces:u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T> Debug for Coffee<T> where T: Debug{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       f.debug_struct("Coffee").field("Kind", &self.kind).field("milk", &self.milk).field("ounces", &self.ounces).finish() 
    }
}

impl<T:Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}",self.ounces,self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavour: String,
    percentage: u32
}

impl Soda {
    fn new(calories: u32,price: f64,flavour: String,) -> Self {
        Self { calories, price, flavour, percentage: 100 }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavour {}, Calories {}",self.flavour,self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Calories: {} Price: {} Flavour: {}  percentage: {}",self.calories,self.flavour,self.price,self.percentage)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self { calories: self.calories, price: self.price, flavour: self.flavour.clone(), percentage:self.percentage }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

fn main() {
    let mut latte = Coffee::new("meow", Milk::Almond, 3);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new("SSS".to_string(), Milk::Oat, 5);
    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(100, 1.0, "Cherry".to_string());
    println!("{}",pepsi);

    let mut coke = pepsi.clone();
    println!("{}",coke==pepsi);
    coke.consume();

    println!("{coke:?}");



}