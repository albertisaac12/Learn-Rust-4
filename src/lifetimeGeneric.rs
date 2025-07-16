fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] { // 'a (tick a) is the full name it represents a generic lifetime, also it represents a future lifetime or a hypothetical lifetime 
    &items[..2]

    // we are basically telling that the items parameter lifetime is related to the return value life time in this case
    // 

    // 'a represents is some generic lifetime, some hypothetical lifetime, a future lifetime that we do not know yet in advance, or that has the potential to vary.

    // We are saying for some generic hypothetical lifetime called 'a, the returned reference must live
    // within the lifetime of the referent that 'items' is a reference to, because we have also marked this with 'a


    // Final conclusion => returned reference should live for as long as the 

    // "For some generic hypothetical lifetime called 'a, the returned reference must live within the lifetime of the referent that items is a reference to, because we have also marked this with 'a."

    // “For some generic lifetime 'a, the function accepts a reference items: &'a [String] to data that lives at least 'a, and it returns a reference to part of that data — which must also live at most 'a.”

    // "I’m returning a piece of data borrowed from the input — not something new — and I will never let that reference outlive the thing it was borrowed from."

    // Lifetimes in Rust apply only to references (&T or &mut T) — not to owned values.
}

fn main() {
    let cities = vec![String::from("London"),String::from("New York"),String::from("Barcelona")];

    let two_cities = select_first_two_elements(&cities);
    // let nn = cities;
    println!("{two_cities:?}");

    {
        let coffies = [String::from("Latte"),String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffies);
        println!("{two_coffees:?}");

    }
}

fn blah<T>(a:T) {

}


/*

    A concrete lifetime is the region of code that a value exists in the program (the time it lives in its memory address).

    A generic lifetime is more abstract. It is a hypothetical lifetime, a non-specific lifetime, a future lifetime that can vary.

    We can annotate generic lifetimes in our code. This enables functions that are flexible enough to handle varying lifetimes.

    A lifetime annotation is a name or label for a lifetime.

    lifetime annotation don't change the reference's lifetime. They don't affect the logic in any way.

    A lifetime annotation is a piece of metadata that we provide to the borrow checker so that it can validate that references are valid.

*/