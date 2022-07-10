use std::rc::Rc;

enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // let a = List::Cons(
    //     5, Box::new(
    //         List::Cons(
    //             10, Box::new(List::Nil))
    //     )
    // );

    // let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a));

    let a = Rc::new(
        List::Cons(
            5, Rc::new(
                List::Cons(10, Rc::new(List::Nil))
            )
        )
    );
    // 数据深度拷贝
    // a.clone();
    println!("count afer create a:{}", Rc::strong_count(&a));

    // Rc::clone 不进行深度拷贝
    let b = List::Cons(3, Rc::clone(&a));
    println!("count afer create b:{}", Rc::strong_count(&a));

    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count afer create c:{}", Rc::strong_count(&a));
    }
    println!("count afer drop c:{}", Rc::strong_count(&a));
}
