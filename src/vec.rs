fn main() {
    let mut vec = Vec::<i32>::new();
    vec.push(1);
    vec.pop();

    let vec1 = vec![0,1,2,3,4];

    vec.insert(0, 1);

    let k = vec.remove(0);

    // vec.get(0);

    let mut v2 = vec!["a".to_string(),"b".to_string(),"c".to_string()];

    let s1 = &mut v2[0];
    // let s2 = &mut v2[0];
    s1.push_str(" meow");

    println!("{v2:#?}");

    let mut vc = Vec::<&str>::with_capacity(4);
    println!("Length {} Capacity {} ",vc.len(), vc.capacity());
    
    vc.push("summer");
    vc.push("summer");
    vc.push("summer");
    vc.push("summer");
    
    println!("Length {} Capacity {} ",vc.len(), vc.capacity());
    
    vc.push("5");
    println!("Length {} Capacity {} ",vc.len(), vc.capacity());
    


}