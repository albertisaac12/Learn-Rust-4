fn main() {
    // <> within the angle brackets we define the generic.
    // A generic is a type Argument 

    // The range type is not available in the top level of file and is nested in a module

    let f:std::ops::RangeInclusive<char> = 'a'..='z';


    let mc = {
        2+2
    };


    
}

// #[allow(unused_variables)]
fn checkString(abc:&str) ->(bool,bool) {

    for i in abc.trim().as_bytes().iter().enumerate() {
       let mc = if i.1 == &b'a' {
            (true,true)
        }else {
            (false,false)
        };
    }

   let bc = if abc.contains('a') && abc.contains('z') {
    (true,true)
   } else {
    (false,false)
   };

   bc



}

// 7702 => 3074