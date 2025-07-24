use std::io::stdin;
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

    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold")
    };
    
    let clo = || {
        let mut buff = String::new();
        println!("Enter Your Password");
        stdin().read_line(&mut buff).expect("Failed to read");
        buff = buff.trim().to_string();
        buff
    };

    // println!("{}",vault.unlock(clo).unwrap_or("Wrong Password, Vault Destroyed".to_string()));


    let mut game_console = String::from("PlayStation");
    game_console.retain(|character| character != 'a');



    let locations = [Location {name: String::from("Enchanted Forest"),treasures: 5},Location{name: String::from("Mystic Mountain"),treasures: 10}];

    let map = Map {
        location: &locations
    };
    let mut total_treasures =0;
    let action = |location: &Location| {
        total_treasures += location.treasures;
    };
    map.explore(action);

    println!("{total_treasures}");

    let mut Location_names = Vec::<String>::new();
    map.explore(|location|{
        Location_names.push(location.name.clone());
    });


    let closure= || println!("hellooo");
    execute_thrice(closure); 

    execute_thrice(bake_cake); // An Fn can accept a regular function un-invoked that does not consume of mutate the variable.

    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new);
    println!("{:?}", collection);

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




    // Method that will except a closure as an argument
    #[derive(Debug)] 
    struct Vault {
        password: String,
        treasure: String
    }

    impl Vault {
        fn unlock<F>(self,procedure: F) -> Option<String> where F: FnOnce()-> String {
            
            let user_password = procedure();
            if user_password == self.password {
                Some(self.treasure)
            } else{
                None
            }
        }
    }


    struct Location {
        name: String,
        treasures: u32
    }

    struct Map<'a> {
        location: &'a [Location]
    }

    impl<'a> Map<'a> {
        fn explore<F>(&self,mut action: F) where F: FnMut(&Location) {
            let final_index = self.location.len() -1;
            let mut current_index = 0 ;
            while current_index<= final_index {
                let current_location = &self.location[current_index];
                action(current_location);
                current_index+=1;
            }
        }
    }



fn execute_thrice<F>(procedure:F) where F: Fn() {
    procedure();
    procedure();
    procedure();
}


fn bake_cake() {
    println!("Cake Baked");
}