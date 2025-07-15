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
    
}

// CONCRETE LIFE TIME : = life time of a variable is clearly defined its concrete
// lifetime is basically for a compiler to understand when the variable came into existence and when the existence ends

// Reference will not outlive the data its referencing too, if it did the reference would turn to a dangling reference so the compiler does not allow it

// A reference is contained within the lifetime of a variable/ data it is referring to.

// The borrow checker is the part of the rust compiler that validates that all borrows (i.e references) are valid



/*

    Non-Lexical Lifetimes

    The word lexical Means lasting until the end of the block. Non-Lexical means not lasting until the end of the block

*/