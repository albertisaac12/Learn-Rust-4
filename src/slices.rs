fn main(){
    // rust permits mutable slice of array

    let mut my_array = [1,2,3,4,5];

    let mut myslice = &mut my_array[2..4];

    myslice[0] = 6;
    println!("{my_array:?}");
    
}

// the range is rather that off the bytes and not that off the index , in english characters 1 byte is one character


// &String can be converted to &str while the converse is not true

// rust does not permit mutable slice of Strings