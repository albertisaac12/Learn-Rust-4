#[derive(Debug)]
struct time<T> {
    minutes:T,
    hours : f64
}

enum blah<T> {
    plain,
    abc(T)
}

// struct ss<T>; // Will throw an error


fn main() {

    let mut now = time::<f64> {
        minutes: 32 as f64,
        hours: 1.0
    };

    println!("{}", identity::<f64>(5.0)); // turbofish operator

    let mut bc:[&str;3] = ["aa","aa","aa"];


    // let plain = blah::plain;  // will give an error
    
    // do this instead
    let dd = blah::<String>::plain;
    let mut dd:blah<String> = blah::plain;
    dd = blah::abc("String".to_string()); // compiler will now enforce T to be String
}


fn identity<T>(value:T) ->T { // generic function
    value
}


// impl<K:Into<f64>> time<K> {

//     fn changeTime(&mut self, other: &K) {
//         self.hours = (*other).into();
//         self.minutes = *other;
//     }

// }

// we cannot move a value behind a shared reference (a shared refrence is simply & and &mut)

// we want to implement an impl block for any type of T rather than restricting it to one single specific type


// if impl<T> the T here was not present and we simply use time<T> this will not work because inside time<T> the T is supposed to be a concrete type so what impl<T> does is it basically says 
// "Now the T inside impl<T> is a generic type for the entire impl block and time<T> says we are implementing an impl for type T"
impl<T> time<T> { 

}