use std::cell::RefCell;

use std::rc::Weak;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(
        Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        }    
    );

    // 获取不可变所有权并通过upgrade将Weak<Node>转化为Rc<Node>
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(
        Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(
                vec![Rc::clone(&leaf)]
            )
        }
    );
    
    // 通过downgrade 将Rc<Node> 转化为Weak<Node>
    let weak_branch = Rc::downgrade(&branch);
    *leaf.parent.borrow_mut() = weak_branch;

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    println!("leaf parent = {:#?}", branch);
    println!("Hello, world!");
}
