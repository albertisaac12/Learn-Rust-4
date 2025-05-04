fn main() {
    let bytes = b"hi"; // the b here converts the hi to a u8 array.

    for byte in bytes {
        println!("{:?}", byte);
    }

    // lets try to iterate over bytes and then enumerate each element individually

    println!(
        "There might be a space at {:?}",
        iter_with_enumerate(&("meow meow".to_string()))
    );

    /*//////////////////////////////////////////////////////////////
                                 SLICES
    ////////////////////////////////////////////////////////////// */

    let s = String::from("To be Converted To Sli1ce");

    let slice1 = &s[..4]; // the upper bound is not included also something to note is that the slice is a &str

    println!("{:?}", slice1);

    println!("{}", s.len()); // results in 25

    // if i did something like below it will be a error

    // let slice2 = &s[..27];

    // println!("{:?}",slice2); // this code will panic

    /*//////////////////////////////////////////////////////////////
                              ARRAY-SLICE
    ////////////////////////////////////////////////////////////// */

    let arr = [1, 2, 3, 4, 5];

    let arrSlice = &arr[1..];

    println!("{:?}", arrSlice);
}

fn iter_with_enumerate(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// note for loops evaluate to a unit type

// Slice
