use std::ops::{Add};

#[derive(Debug)]
struct Lunch {
    cost: f64
}

impl Add for Lunch {
    type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost
    }
}

fn add_two_numbers<T>(a:T,b:T) -> T where T: Add<Output = T>{
    a+b
}


fn main() {
    let l1 = Lunch {cost: 100.0 };
    let l2 = Lunch {cost : 20.1};

    println!("{:.2}",l1+l2);

    println!("{}",add_two_numbers(4, 5));

}

// An associated type is a placeholder for a type that is required within a trait