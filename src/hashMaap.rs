use std::collections::{btree_map::Values, HashMap};
fn main() {
    let mut hp = HashMap::<u32,u32>::new();

    hp.insert(1, 1);
    hp.insert(2, 1);
    hp.insert(3, 1);

    if let Some(value) = hp.remove(&3) {
        println!("This value was removed {value}");
    }

    let mut coffee_pairing: HashMap<&String,&String> =  HashMap::new();

    let drink = String::from("Coffee");
    let milk = String::from("Milk");

    coffee_pairing.insert(&drink,&milk);
    // drop(milk);
   // coffee_pairing.get(&drink);


    let val = hp[&1];

    let val = coffee_pairing.get(&drink).copied().unwrap_or(&"Unknown Milk".to_string());
}
// HashMap type lives on the Heap 
// HashMap type takes ownership of both key and value
// length of HashMap is the count of total key value pairs