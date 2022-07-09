fn main() {
    println!("Hello, world!");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    x
}