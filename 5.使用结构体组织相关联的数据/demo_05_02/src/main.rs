#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    // let rect = (30, 50);
    let square = Rectangle::square(20);

    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        length: 10,
    };

    let rect3 = Rectangle {
        width: 35,
        length: 55,
    };

    println!("{}", rect.can_hold(&rect2));
    println!("{}", rect.can_hold(&rect3));

    println!("{:#?}", square);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
