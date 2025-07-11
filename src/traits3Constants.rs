trait Investment<T> {
    fn amount(&self) -> T; // is kind off a getter function
    
    // fn set_amount(&mut self,new_amount: f64); // kind Off a setter function 
    
    fn double_amount(&mut self);
}

trait Taxable: Investment<f64>{

   const TAX_RATE:f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }

}

#[derive(Debug)]
struct Income {
    amount: f64
} 

impl Taxable for Income {
    // fn tax_bill(&self) -> f64 {
    //     self.amount * Self::TAX_RATE
    // }
}

#[derive(Debug)]
struct Bonous {
    value: f64
} 

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount = self.amount * 2.0;
    }

    // fn set_amount(&mut self,new_amount: f64) {
    //     self.amount = new_amount;
    // }
}

impl Investment<f64> for Bonous {
    fn amount(&self) -> f64 {
        self.value
    }

    // fn set_amount(&mut self,new_amount: f64) {
    //     self.value = new_amount;
    // }

    fn double_amount(&mut self) {
        self.value = self.value * 2.0;
    }
}


impl Taxable for Bonous {
    const TAX_RATE:f64 = 0.5; // This will override 

    // fn tax_bill(&self) -> f64 {
    //     self.amount * Self::TAX_RATE
    // }

   
}


#[derive(Debug)]
struct QualityTime {
    minutes : u32
}

impl  Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes = self.minutes *2; 
    }
}
fn main() {
    let mut my_income = Income { amount: 1000.0 };
    println!("Initial Income: {:?}", my_income);
    println!("Income amount(): {}", my_income.amount());
    println!("Income tax_bill(): {}", my_income.tax_bill());

    my_income.double_amount();
    println!("Income after double_amount(): {:?}", my_income);
    println!("Updated Income tax_bill(): {}", my_income.tax_bill());

    // my_income.set_amount(5000.0);
    println!("Income after set_amount(5000.0): {:?}", my_income);
    println!("Updated Income tax_bill(): {}", my_income.tax_bill());


    let mut my_bonus = Bonous { value: 2000.0 };
    println!("\nInitial Bonus: {:?}", my_bonus);
    println!("Bonus amount(): {}", my_bonus.amount());
    println!("Bonus tax_bill(): {}", my_bonus.tax_bill());

    my_bonus.double_amount();
    println!("Bonus after double_amount(): {:?}", my_bonus);
    println!("Updated Bonus tax_bill(): {}", my_bonus.tax_bill());

    // my_bonus.set_amount(8000.0);
    println!("Bonus after set_amount(8000.0): {:?}", my_bonus);
    println!("Updated Bonus tax_bill(): {}", my_bonus.tax_bill());
}

// supertrait is a trait from which another trait inherits functionality. The parent is called the supertrait and the child is called the subtrait