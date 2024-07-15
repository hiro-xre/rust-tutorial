fn main() {
    let a: Option<String> = Some(String::from("hello"));
    match a.as_ref() {
        Some(x) => println!("{}", x), // move ownership
        None => (),
    }
    println!("{:?}", a);
}
