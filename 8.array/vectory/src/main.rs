enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();

    let v: Vec<i32> = vec![1, 2, 3];

    let v: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Text(String::from("_")),
        SpreadsheetCell::Float(10.12)
    ];
}
