use std::collections::{HashSet, HashMap};
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

    let ee = hp.entry(4).or_insert(5);

    let mut hs1 = HashSet::<&str>::new();
    hs1.insert("a");
    hs1.insert("b");

    let mut hs2 = HashSet::<&str>::new();
    hs2.insert("a");
    hs2.insert("c");

    hs2.insert("c"); // Will not panic but will not insert another c

    println!("{:?}",hs2.union(&hs1));
    println!("{:?}",hs2.difference(&hs1)); // values in hs2 but not in hs1
    println!("{:?}",hs2.symmetric_difference(&hs1)); // values not common in hs2 and hs1

    println!("{:?}",hs2.is_disjoint(&hs1)); // will be false only will be true if there are no common elements

    println!("{:?}",hs2.is_subset(&hs1)); // if all elements in hs2 are present in hs1

    println!("{:?}",hs2.is_superset(&hs1)); // if all elements in hs1 are present in hs2
}
// HashMap type lives on the Heap 
// HashMap type takes ownership of both key and value
// length of HashMap is the count of total key value pairs