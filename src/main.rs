fn main() {
    let s1 = String::from("hello");

    let s2 = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
