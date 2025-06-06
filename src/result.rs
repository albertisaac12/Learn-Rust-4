fn main() {
    let k:Result<i32, i32> = Ok(5);
    let k:Result<i32, i32> = Err(5);

    let text = "50";
    let mc = text.parse::<i32>();

    let res = divide(5.0, 0.0);
    println!("{}",res.is_err());
    let meow = res.unwrap_or(0.0);
    println!("{meow}");

    let my_result = operation(true);

    let content = match my_result {
        Ok(a) => {
            a
        },
        Err(b) => {
            b
        }
    };
    
    // println!("{:?}",my_result); // borrow of partially moved value: `my_result`

    let mut veca:Vec<&str> = vec!["a","b","b"];

    while let Some(a) = veca.pop() {
        println!("{a}");
    }

}

fn divide(numerator: f64,denominator: f64) -> Result<f64,String> {
    if denominator == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(numerator/denominator)
    }
}


fn operation(success:bool) -> Result<String,String> {
    if success {
        Ok("LETSGOOOO".to_string())
    }else {
        Err("Error".to_string())
    }
}

// cavitates of unwrap method on Result<T,E> , the type we are extracting either T or E may or may not implement the Copy trait