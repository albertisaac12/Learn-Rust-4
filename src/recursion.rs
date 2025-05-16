use std::iter::Rev;

fn main() {
    // function calling itself
    let sum = rec(5); 
    // base case is a condition that stops recursion

    let f = fac(5);
    println!("{f}");
}


fn rec(u:u32) {
    if u == 0 {return}
    println!("{u} seconds to blastoff");
    rec(u-1);   
}

fn fac(u:u32)-> u32 {
    if u == 0 {
        return 1;
    }else {
        u * fac(u-1)
    }
}

// remember always the base case will stop the recursion    