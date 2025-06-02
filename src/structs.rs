#[derive(Debug)] // derive means to infer from
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
} // Declaring the struct out side of main makes it available to all functions in this file but not so public that it can be accessed by all other functions.
#[derive(Debug)] // derive means to infer from
struct Song {
    price: f64,
    name: String,
    is_hot: bool,
} // Declaring the struct out side of main makes it available to all functions in this file but not so public that it can be accessed by all other functions.

#[derive(Debug)] // derive means to infer from
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

fn main() {
    let name = "mocha".to_string(); // owner is name
    let mocha = create_coffee(1.5, name, true); // ownership transfered to crate_coffee function => ownership returned to mocha variable
    println!("{:#?}", mocha);
    // println!("{:?}",name);
    let mut latte = Coffee {
        name: "Latte".to_string(),
        ..mocha
    };

    println!("{:?}", latte);
    drink_coffee_reference(&latte);
    println!("{:?}", latte);
    drink_coffee_reference_mutable(&mut latte);
    println!("{:?}", latte);

    let mc = Song {
        name: "jel".to_string(),
        price: 1.5,
        is_hot: true,
    };

    mc.display_song_info();

    let mc = Song::new_song("hi".to_string(), 6.5, true);
    mc.display_song_info();

    let mut mc = Song::new_song("hi".to_string(), 6.5, true);
    mc = mc.change_song_info();
    mc.display_song_info();

    let mc = Song::new_song("hi".to_string(), 6.5, true);
    mc.display_song_info_no_ownership();
    println!("{:?}", mc);

    let mut mc = Song::new_song("hi".to_string(), 6.5, true);
    mc.change_song_info_no_ownership();
    println!("{:?}", mc.name);

    let mc = Song::new_song("hi".to_string(), 6.5, true);
    let nc = Song::new_song("hello".to_string(), 8.5, true);

    println!("{}", mc.compare_name(&nc));

    let mut computer = Computer::new("meow".to_string(), 1, 2);
    computer
        .upgrade_cpu("meow2".to_string())
        .upgrade_memory(10)
        .upgrade_capacity(5);
    println!("{:#?}", computer);
}

fn create_coffee(price: f64, name: String, is_hot: bool) -> Coffee {
    Coffee {
        price,
        name, // ownership given to Coffee Struct
        is_hot,
    }
}

// instance by value immutably/ mutably

fn drink_coffee(coffee: Coffee) {
    // Coffee AFTER ENTERING HERE WILL LOSE ITS OWNERSHIP
    println!("Drinking Coffee, {}", coffee.name);
}

fn drink_coffee_mutable(mut coffee: Coffee) {
    println!("Drinking Coffee, {}", coffee.name);
    coffee.name = "done".to_string();
}

fn drink_coffee_reference(coffee: &Coffee) {
    println!("{}", coffee.name); // auto deref 
}

fn drink_coffee_reference_mutable(coffee: &mut Coffee) {
    coffee.name = "newCoffee".to_string();
}
fn mut_drink_coffee_reference_mutable(coffee: &mut Coffee) {
    coffee.name = "newCoffee".to_string();
    let blah = Coffee {
        name: "jee".to_string(),
        price: 1.1,
        is_hot: false,
    };

    // coffee = &mut blah;
    *coffee = blah;
}

/*//////////////////////////////////////////////////////////////
                             STRUCT METHODS
////////////////////////////////////////////////////////////// */

impl Song {
    fn display_song_info(self: Self) {
        // self by itself means self:Self
        // self , &self , mut self , &mut self
        println!("name : {}", self.name);
        println!("Hot or Not : {}", self.is_hot);
        println!("Price : {}", self.price);
    }
    fn change_song_info(mut self) -> Self {
        // self by itself means self:Self
        // self , &self , mut self , &mut self
        self.name = "meow".to_string();
        self
    }
    fn display_song_info_no_ownership(&self) {
        // self by itself means self:Self
        // self , &self , mut self , &mut self
        println!("name : {}", self.name);
        println!("Hot or Not : {}", self.is_hot);
        println!("Price : {}", self.price);
    }
    fn change_song_info_no_ownership(&mut self) /*-> Self*/
    {
        // self by itself means self:Self
        // self , &self , mut self , &mut self
        self.name = "meow".to_string();
        // *self
    }

    fn new_song(name: String, price: f64, is_hot: bool) -> Self {
        Song {
            price,
            name,
            is_hot,
        }
    }

    fn compare_name(&self, other: &Self) -> bool {
        self.name.len() > other.name.len()
    }
}

// Self is an alais and a nickname to whatever we are implementing on the method on

// BUILDER PATTERN => return reference of the instance

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}
