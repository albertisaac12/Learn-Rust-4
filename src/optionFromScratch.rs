enum myOption {
    Some(i32),
    None,
}

impl myOption {
    fn unwarp(self) -> i32 {
        match self {
            myOption::Some(a) => {
                a
            }
            myOption::None => {
                panic!("vlaaahjjjjhh");
            }
        }
    }
}

fn main() {
    let my = myOption::None;
    my.unwarp();
}