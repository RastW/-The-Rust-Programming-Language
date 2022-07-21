use std::marker::Sync;  // 允许多线程同时访问，lock用
use std::marker::Send;  // 允许线程间转移所有权，channel用

fn main() {
    let s = String::from("hello");
    let s1 = exec(s);

    print!("{}", s);
    print!("{}", s1);
}

fn exec(s: String) -> String{
    s
}