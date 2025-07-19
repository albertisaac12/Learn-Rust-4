
/*
    Elision is the act of omitting something. Lifetime elision means omitting generic lifetime annotations in situations where the
    borrow checker can infer the lifetime relationships automatically

*/


struct DentistAppointment {
    doctor: String,
}

impl DentistAppointment {
    fn book <'a>(&self, check_in_time: &'a str,check_out_time: &str) -> &'a str {
        check_in_time
    }
}

fn main() {

   let name = String::from("Nj Transit");

    let nj_transit = TrainSystem {
        name : &name
    };
    
    println!("{nj_transit:?}");
    
    // An Error Case
    // let nj_transit = {
        
    //     let name = String::from("Nj Transit");
        
    //     TrainSystem {
    //         name : &name // This will be an error because the lifetime of the struct is only valid for as long as life time of the above name
    //     }
    // };
    // println!("{nj_transit:?}");

}

// Rules of Elision

// First Rule: The compiler assigns a lifetime to each parameter that is a reference.

// Second Rule: If there is one reference parameter and the return value is a reference, 
// the borrow checker will infer that their lifetimes are related.

// Third Rule: If there are multiple input references and one of them is &self or &mut self, 
// the lifetime of self is assigned to all returned references.




fn my_awesome_function(value: &i32, second: &str) {

}


fn choose_favorite<'a>(first: &'a str, second: &'a str) -> &'a str { // either one of the reference can be connected to returned reference or only one of them
    let some_condition = true;

    if some_condition {
        first
    } else {
        second
    }

}



// Basically a lifetime tells that a returned reference should at least be valid and should exist or be used after the life time of the referent that the function parameter is referring to

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// fn main() {
//     let s1 = String::from("Hello");
//     let result;
    
//     {
//         let s2 = String::from("Hi");
//         result = longest(&s1, &s2); // ❌ Problem here
//     } // s2 is dropped here!

//     println!("Longest: {}", result); // ❌ Use-after-free risk
// }


/*  

    Essentially the concrete life time for the longest function will be the shorter of both the variables passed in and also to be precise,
    the overlapping area will be the concrete lifetime.
*/


/*
    Lifetimes in Struct

    // data inside struct should be de allocated after the struct variable itself

*/

#[derive(Debug)]
struct TrainSystem<'a> {
    name: &'a str
    /*
    TrainSystem name field should live as long what it is referring too and should not be valid after the referent is dropped
    Life time of struct is connected to the life time of the name field's refferent
    */
}

#[derive(Debug)]
struct TravelPlan<'a> {
    from: &'a str,
    to: &'a str

    // Like regular function parameters the Struct instance will be only valid for the overlapping region of the both references
}



fn sample(a: i32, b: &String, c: Vec<String>) -> &String {
    b
}


// fn main() {
//     let mut fruits = vec!["Apples", "Strawberries", "Pears"];
//     let fruit_ref_1 = &mut fruits;
//     let fruit_ref_2 = &mut fruits;
//     println!("{fruit_ref_1:?}"); // will give error because of NLL rule
//     println!("{fruit_ref_2:?}");
// }

// Lecture 337



