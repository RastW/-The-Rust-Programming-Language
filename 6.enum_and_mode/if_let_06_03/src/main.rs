fn main() {
    let v = Some(0u8);

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others")
    }
}
