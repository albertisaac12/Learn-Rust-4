fn main() {


    let a = 1;

    {
        let b = 2; // life time of this scope 
    } // b gets dropped here

    // When the rust transfers ownership the lifetime ends

    let c = String::from("jjj"); // Life time of c begins here
    let d = c; // lifetime of c ends here and life time of d begins here 
    drop(d); // life time of d ends here


    let dog = String::from("Watson");

    let my_pet = &dog;


    let mut x = String::from("hello");
    let r = &x;
    println!("{}", r);    // Last use of r
    x.push_str(" world"); // ✅ OK, NLL sees r is no longer used
    println!("{}",x);

    /* 
        let cities = vec![String::from("dddd"),String::from("adadasd"),String::from("aaaa")];
        let fav = &cities[0..2];
        drop(cities);
        println!("{:?}",fav); // dangling reference
    
        
    let some_cities = {
        let cities = vec![String::from("dddd"),String::from("adadasd"),String::from("aaaa")];
        &cities[0..2]
    }; // cities get dropped here invalidating the reference and boom voila we are trying to return a dangling reference

    */

    // let cities = vec![String::from("dddd"),String::from("adadasd"),String::from("aaaa")];
    // let fav = &cities[0..2];
    // let same_cities = cities; // since some borrow for cities exsist we cannot move it to some_cities
    // println!("{:?}",fav);


    
    // let cities = vec![String::from("dddd"),String::from("adadasd"),String::from("aaaa")];
    // let same_cities = cities;
    // let fav = &cities[0..2]; // borrow of moved value the cities is now moved into same_cities
    // println!("{:?}",fav);  

    let cities = vec![String::from("dddd"),String::from("adadasd"),String::from("aaaa")];
    let fav = &cities[0..2]; // lifetime begins and end here
    let same_cities = cities;
    // println!("{:?}",fav);   


    let cities = vec![String::from("dddd"),String::from("adadasd"),String::from("aaaa")];
    let two_cities = select_first_two_elements(&cities); 
    println!("{:?}",two_cities);
    
    {   

        let cities = vec![String::from("sdfsdf"),String::from("sdfsdf"),String::from("sdfsdfs")];
        let two_cities = select_first_two_elements(&cities); 
        // drop(cities); // Uncomment to simulate the error
        println!("{:?}",two_cities);
        // The cities is being dropped and since two_cities is essentially a reference to the cities this caused issues but wait how does rust know what we are referring to ?
        
        // The items in the select_first_two_elements(items: &[String]) -> &[String] {} . Basically when we passed the reference of cities into function it became items and that items is now tied with the &[String] as return value which then when the compiler looks at gets tied to two_cities variable and that is at end essentially linked to cities
        
        // Essentially the reference should live within the life time of referent and the &[String] being returned is tied to items which is tied to cities and hence the code works
    }


}

// CONCRETE LIFE TIME : = life time of a variable is clearly defined its concrete
// lifetime is basically for a compiler to understand when the variable came into existence and when the existence ends

// Reference will not outlive the data its referencing too, if it did the reference would turn to a dangling reference so the compiler does not allow it

// A reference is contained within the lifetime of a variable/ data it is referring to.

// The borrow checker is the part of the rust compiler that validates that all borrows (i.e references) are valid



/*

    Non-Lexical Lifetimes
    The word lexical Means lasting until the end of the block. Non-Lexical means not lasting until the end of the block
    The borrow checker treats the end of a reference's lifetime as the last place it is used; a reference has non-lexical scope.


    if the reference was created and never used rust assumes that the reference could be used in the future hence and the lifetime will move on until it will be used.

    when the lifetime of a reference outlives the lifetime of the data/variable its borrowing from it creates a dangling reference 

*/








/*
    A function cannot return a reference to any variable created in the function scope
    A function cannot return a reference to a owner value (parametes)
*/
 
// fn create() -> &i32 {
//     let age = 100;
//     &age
// }


// fn create_slcie(items: Vec<i32) -> &[i32] {

//     &items

// }



fn select_first_two_elements(items: &[String]) -> &[String] {
    let selected_items = &items[..2];
    // println!("{:?}",selected_items);

    selected_items
}























