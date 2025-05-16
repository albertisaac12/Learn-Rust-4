fn main() {
    let value:i32 = -15;
    println!("{}", value.pow(2));

    let st = "      hello       ";
    println!("{}",st.trim());


    let pi:f64 = 3.141565656589668222;
    println!("{}",pi);
    println!("{}",pi.floor());
    println!("{}",pi.ceil());
    println!("{}",pi.round());
    
    println!("{:.2}",pi);
    println!("{0:.4}",pi);
}