enum Message {
    Quit,
    Move {x: i32, y: i32},          // 匿名结构体
    Write(String),                  // 字符串
    ChangeColor(i32, i32, i32),     // 元组
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move{x: 12, x: 32};
    let w = Message::Write(String::from("hello world"));
    let c = Message::ChangeColor{0, 255, 255};

    m.call();
}