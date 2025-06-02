fn main() {
    let str = String::from("hi");
    let mut mc = str;
    // println!("{str}"); // will result in an error borrow of moved value meaning the varaible mc now owns the value of str and str is dropped
    println!("{mc}");
    mc.push_str(" meow");
    println!("{mc}");

    let bc = mc.clone(); // the String type implements the Clone trait
    println!("{}", mc);
    drop(mc); // drop can be called manually too
    // println!("{mc}");

    let mut ff = String::new();
    ff.push_str("hie");

    // let mut uu = &ff;
    // *uu = String::from("hello");

    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;

    let my_heap_value = String::from("maomao");
    let my_heap_reference = &my_heap_value;
    // reference is an example of a pointer
    // drop(my_heap_value);
    println!("{:#?}", *my_heap_reference);
    println!("{:#?}", my_heap_reference);

    let mut ice_cream = "cookies and cream";
    let dessert = ice_cream;
    ice_cream = "mmmmm";
    println!("{}", ice_cream);

    println!("{}", dessert);

    let mut meal = String::from("burger");
    meal = add_fries(meal);
    println!("{}", meal);
}

fn add_fries(mut meal: String) -> String {
    meal.push_str("and Fries");
    meal
}

fn add_fries_mut(meal: &mut String) {
    (*meal).push_str("fries Added");
}
// What is ownership ?

// A variable can be a owner.
// A parameter can be a owner.
// A tuple and array own their values.
// Every value in rust program has at least one owner at any given point of time.
// The owner can be changed at any point in the program.
// The owner is who/ what is responsible for cleaning up a piece of data when its no longer in use.
// A composite type is a type combined of multiple types.

// When rust needs to store an element in memory it either chooses the stack or heap

// STACK
/*
    => fast.
    => fixed predictable constant size, the size must be known at the compile time.

    => LIFO
    => push
    =>pop

*/

// HEAP
/*
    => slow.
    => dynamic data, can change over time during the entire period of the program.

    => A program called memory allocator finds an empty spot large enough to store the data
    => The heap is a large are of storage space
    => heap is for data whose size is not known at compile time

    => The memory allocator returns a reference of the empty location (memory address).

    Note: since the size and value of reference is known it can be stored on a stack.
*/

// pushing to the stack is faster than the heap , reading data is also fast in stack
// The owner knows how to cleanup the data => it is cleaned up when the owner goes out of the scope.

// Any type implementing the copy trait will be cloned on the stack

// drop function will only work on heap allocated memory.

// reference is cheap to store

// reference is only valid for as long as referee is valid.

// An operation is a symbol that applies an operation to a value

// To dereference means to access the data at the memory address that the reference points to.

// The function parameters can be both mutable and immutable they also can take a reference, the reference can be either be mutable or immutable

/*
    Function Parameter Understanding
    Parameter => cat

    mut cat => the cat can change meaning cat can be assigned a new value
    cat: & => cat is a reference
    cat :&mut => reference that can change a cat

    Presidency order of . is grater than * so when you do this
    *cat.push_str("meow meow"); // this will throw an error because the . will be evaluated first and then we try to dereference whats left off it.

    for the actual arguments being passed in use &.
*/

// meal: String
// mut meal: String
// meal: &String
// meal: &mut String

// reference types also implement the copy trait
