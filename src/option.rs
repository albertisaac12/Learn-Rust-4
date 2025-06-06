fn main() {
    // let k = None;
    let mut f = Some(64);
    f = None;

    let k:Option<f64> = None;

    let mm = Option::Some(5);

    let mmn = Option::<i32>::None;

    let arr = [1,2,3,4,5,6,4];

    let m = arr.get(100);

    if let Some(a) = m {
        println!("{}",a);
    } else {
        println!("Index out of bounds");
    }

    // the unwarp method attempts to extract the associated data out of the Some variant
    let valid = mm.unwrap(); // if the variant is None it will Panic
    println!("{valid}");

    let valid = mmn.unwrap_or(5);
    println!("{valid}");

    // let valid = mmn.expect("Its a None value");
    // println!("{mmn:?}");

    let mc = is_item_in_stock(true, false);

    println!("{:?}", mc);
}


fn is_item_in_stock(item_in_system: bool, item_in_stock: bool)->Option<bool> {
    if item_in_system && item_in_stock {
        Some(true)
    }else if item_in_system{
        Some(false)
    }else {
        None
    }
}