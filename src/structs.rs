#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool
} // Declaring the struct out side of main makes it available to all functions in this file but not so public that it can be accessed by all other functions.

fn main() {
    let name ="mocha".to_string(); // owner is name
    let mocha = create_coffee(1.5,name, true); // ownership transfered to crate_coffee function => ownership returned to mocha variable
    println!("{:#?}",mocha);
    // println!("{:?}",name);
    let mut latte = Coffee {
        name:"Latte".to_string(),
        ..mocha
    };

    println!("{:?}",latte);
    drink_coffee_reference(&latte);
    println!("{:?}",latte);
    drink_coffee_reference_mutable(&mut latte);
    println!("{:?}",latte);


}

fn create_coffee(price:f64,name:String,is_hot:bool) -> Coffee {
    Coffee {
        price,
        name, // ownership given to Coffee Struct 
        is_hot,
    }
}

// instance by value immutably/ mutably 

fn drink_coffee(coffee:Coffee) { // Coffee AFTER ENTERING HERE WILL LOSE ITS OWNERSHIP
    println!("Drinking Coffee, {}", coffee.name);
}

fn drink_coffee_mutable(mut coffee: Coffee) {
    println!("Drinking Coffee, {}", coffee.name);
    coffee.name = "done".to_string();
}

fn drink_coffee_reference(coffee: &Coffee) {
    println!("{}",coffee.name); // auto deref 
}

fn drink_coffee_reference_mutable(coffee: &mut Coffee) {
    coffee.name = "newCoffee".to_string();
}
fn mut_drink_coffee_reference_mutable( coffee: &mut Coffee) {
    coffee.name = "newCoffee".to_string();
    let mut blah = Coffee {
        name:"jee".to_string(),
        price: 1.1,
        is_hot: false
    };

    // coffee = &mut blah;
    *coffee = blah;
}

