
fn main() {

    let multiplier = 5;


    let multiply_by = |var:u32| var*multiplier;
    
    let four_times_five = multiply_by(4);
    
    println!("{}",four_times_five);
    
    let bc = || 5; // shortcut


    let multiply_by = |var| var*multiplier; // the assignment of datatype will happen after first invocation
    multiply_by(2);

    let mut some_String = String::from("Hello");
    let immut = || println!("{}",some_String);
    immut();

    let mut change = || some_String= some_String.to_lowercase(); // here is the mutable reference (This definition captures a mutable reference)
    // println!("{}",some_String);  // This will throw an error because of the borrow rules we cannot have other immutable references after a mutable reference
    change();
    // println!("{}",some_String);  // still will throw error
    change();

    let owner = || {
        let mc = some_String;
    };
    // println!("{}",some_String); // will be an error as owner consumed some_String
    owner();
    // owner();  // This will be an error we are trying to move an already moved value

    let first_name = String::from("aLICE");
    let mut capture_string = || {
        drop(first_name);
    };
    // capture_string();
    // let blah = &mut capture_string;
    // blah();
    // let owner = capture_string();


    // move keyword

    let name = String::from("meowww");
    let capture_string = move || {  // move keyword will move everything it sees in the function body
        println!("{name}") // since the name variable was not consumed either by a different variable or was not moved as a return value we can reuse it again
    };
    // println!("{name}"); // will throw an error because name was moved inside
    capture_string();
    capture_string();
    capture_string();
    capture_string();

    // Practical example of clousers

    None.unwrap_or_else(|| println!("INSIDE UNWRAP_OR_ELSE"));

    let k = Some(4).unwrap_or_else(|| 5 );



}

// Functional programming treats a function like any other value in a program => we can use function as a parameter as a variable etc

    // +---------+      ┌───────────────────────────────────────────────┐
    // |   Fn    | ◄────┤ Immutably borrows captured variables (&T)     │
    // +---------+      │ Can be called multiple times without mutation │
    //      ▲           └───────────────────────────────────────────────┘
    //      │
    // +---------+      ┌──────────────────────────────────────────────┐
    // |  FnMut  | ◄────┤ Mutably borrows captured variables (&mut T)  │
    // +---------+      │ Can be called multiple times, may mutate     │
    //      ▲           └──────────────────────────────────────────────┘
    //      │
    // +---------+      ┌──────────────────────────────────────────────┐
    // | FnOnce  | ◄────┤ Takes ownership of captured variables (T)    │
    // +---------+      │ Can be called once; consumes the closure     │
    //                  └──────────────────────────────────────────────┘



// the same order from Udemy 

    // Fn <- FnMut <- FnOnce 

    // Fn => closures captures values by immutable reference (read -only) or doesn't capture anything at all, closure can be invoked multiple times
    // FnMut => captures values by mutable reference, closure can be invoked multiple times
    // FnOnce => closure captures values by move (transferring ownership), Closure will be invoked once.

    // FnMut is a sub trait of FnOnce, Fn is a sub trait of FnMut


    // the lifetime of a mutable reference the FnMut takes is active till the last invocation

    // with FnOnce the move occurs directly during the declaration and not during the invocation