fn main() {
    let arr = [true; 4];
    let bc = arr[1];
    // What happens here is since the bool implements the Copy Trait a copy of the value will be made and be assigned to the bc;

    // let arr = ["ss";5];
    // let mc = arr[0];
    // println!("{arr:?}");
    // println!("{mc:?}");

    // let arr = ["hello".to_string().clone();5]; // will give an error , the reason is that this syntax with ;5 the rust will try to make use of the Copy Trait again but it is not available.``

    let mut arr = [
        String::from("hello"),
        String::from("meow"),
        String::from("bye"),
    ];
    // let a1 = arr[0]; // rust will try to make a copy of the String from within the array into the a1 variable here resulting in an error
    let a1 = arr[0].clone();
    let b1 = &mut arr[0];
    b1.push_str(" meow meow meow");

    println!("{b1}");
    let abc = (1, 2, String::from("hello"));

    // let (a,b,c) = abc;
    // println!("{abc:?} {c}"); // error

    let mc = abc.2;
    println!("{mc}"); // this will work because the lifetime of abc ended in the above line
    // println!("{}",abc.2); // this will again say borrow of moved value beacuse in line 25 the value was moved to mc if not partially mc.

    // solution
    let abc = (1, 2, String::from("hello"));
    let mc = abc.2.clone(); // method 1
    let mc = &abc.2; // method 2
}
