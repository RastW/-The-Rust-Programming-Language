fn main() {
    // gives_ownership 将返回值赋予s1
    let s1 = gives_ownership();

    // s2进入作用域
    let s2 = String::from("hello");

    // s2被移动到takes_and_gives_back中，它也将返回值移给s3
    let s3 = takes_and_gives_back(s2);


    let ss1 = String::from("hello");

    let (ss2, len) = calculate_length(ss1);

    println!("The length of: '{}' is {}", ss2, len);

}
    // 这里，s3移除作用域并被废弃。
    // s2也移出作用域，但它已被移走，所以什么也不会发生。
    // s1移出作用域并被废弃。

fn gives_ownership() -> String {
    // some_string 进入作用域
    let some_string = String::from("hello rust");

    // 返回some_string并移出给调用的参数
    some_string
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 使用元组多返回值
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
