fn double_the_length<T>(input:&Vec<T>) -> usize {
    input.len() * 2 as usize
}

fn last_two<T>(input: &[T]) -> &[T] {
    &input[input.len()-2..]
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{}",announcement);
    &text[0..5]
}

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        return first;
    } else if second.contains(target) {
        return second;
    }
    "Both string slice don't hold target"
}

fn main() {
    println!("{}",double_the_length(&vec![1,2,3]));
    println!("{:?}",last_two(&vec![1,2,3]));
    println!("{}",first_five("refrigerator", "Hello"));
    println!("{}",find_string_that_has_content("programming", "dining", "gram"));
}