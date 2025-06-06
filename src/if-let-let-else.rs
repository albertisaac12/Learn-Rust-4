
#[derive(Debug)]
enum test1 {
    a,
    b,
    c(String),
    d{ kind: String},
}

fn main() {
    let ac = test1::c("hello".to_string());
    ac.testmatch();
    let bc = test1::b;
    bc.testmatch();

    let my_bev = test1::a;

    // if let test1::c(a) = my_bev { // if my_bev is the variant of enum run the block of code
    //     println!("{a:?}");
    // };

    let test1::d{kind: mc} = my_bev  else {
        println!("hiiii ");
        return ();
    };

    println!(" This is mc {mc}");


}

impl test1 {
    
    fn testmatch(&self) {
        match self {
            test1::a | test1::b => {
                println!("a or b");
            },
            test1::c(mc) => {
                println!("{mc:?}");
            }
            _ => {

            }
        }
    }

}

// if let is used when we want to only match one type of variant
// let else is opposite it executes when a dynamic value does not match the hardcoded variant 