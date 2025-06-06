fn main() {
    // rust permits mutable slice of array

    let mut my_array = [1, 2, 3, 4, 5];

    let mut myslice = &mut my_array[2..4];

    myslice[0] = 6;
    println!("{my_array:?}");


    meoww(&my_array);
}


fn meoww(i:&[i32])  {
    println!("{i:?}");
}

// the range is rather that off the bytes and not that off the index , in english characters 1 byte is one character

// &String can be converted to &str while the converse is not true

// rust does not permit mutable slice of Strings

/*
    let abc:&str = "hiiii";
    let bc:&str = &abc[2..];
    here the bc variable is not creating a reference to the reference abc but it uses abc as a base to create its own independent reference 
    meaning it uses abc once to locate the chunk of the memory and later create its own reference
*/

// the length of the string slice refers to the count of its bytes and not of its characters