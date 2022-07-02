use my_project::traits::summary::Summary;
use std::{
    fmt::{format, Debug, Display},
    iter::Sum,
};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn tongyong(&self) {}
}

impl Pair<i32> {}

// Pair的T为实现了Display + PartialOrd 的实现时，才拥有的方法
impl<T: Display + PartialOrd> Pair<T> {
    fn i(&self) {}
}

struct NewArticle {}

// NewArticle 实现了summary
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        "NewArticle".to_string()
    }
}

struct Tweet {}

// Tweet 实现了summary, summary中的所有方法都有默认实现，可以不写
impl Summary for Tweet {}

pub fn notify(item: impl Summary) {}

pub fn notify2(item: impl Summary + Display) {}

pub fn notifyy<T: Summary>(item: T) {}

pub fn notifyy2<T: Summary + Display>(item: T) {}

pub fn notifyyy<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("Brank news: {}", a.summarize())
}

pub fn notifyyy2<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Brank news: {}", a.summarize())
}

pub fn notifyyyy(item: impl Summary) -> impl Summary {
    item
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

fn largest3<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    largest
}

trait Dign {
    fn vv();

    // fn calculate<T: Copy + Clone, U: Unpin>(a: &T, b: U) -> (&T, i32);

    fn calculate(&self, a: impl Summary, b: impl Copy + Clone) -> (&str ,i32);
}

// 覆盖实现
// 所有实现了Dign的 Summary 实现体都拥有的方法
// 所有满足Summary 这个trait的类型都实现了Dign这个trait
impl<T: my_project::traits::summary::Summary> Dign for T {
    fn vv() {}

    fn calculate(&self, a: impl Summary, b: impl Copy + Clone) -> (&str ,i32) {
        ("calculate result", 64)
    }
}

fn main() {
    println!("Hello, world!");

    let number_list = vec![1, 2, 3];
    let k = largest(&number_list);
}
