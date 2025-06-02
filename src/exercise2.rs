fn main() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let last_three = &mut cereals[3..];
    print_str(last_three);
    last_three[1] = String::from("Captain Crunch");

    let first_two = &cereals[0..2]; // immutable reference to the cereals variable
    print_str(first_two);

    let mid_three = &cereals[1..4];
    print_str(mid_three);

    let cookie_crisp = &first_two[0];
    let cookie = &cookie_crisp[0..5];
    println!("{cookie}");

    // let cocoa_puffs = &last_three[1];
    // let puffs = &cocoa_puffs[6..];
    // println!("{puffs}");
}

fn print_str(val: &[String]) {
    println!("{val:?}");
}

/*

    Having an mutable reference after an immutable reference is totally fine until and unless their you use an immutable variable after a. meaning use of an immutable reference after the
    If you have a &mut T, you cannot create any &T (even to a part of it) until the &mut T is no longer used.

*/
