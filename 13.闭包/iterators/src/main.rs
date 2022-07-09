struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let name_vec: Vec<String> = vec![String::from("1234"), String::from("ksjfk")];
    let size_vec: Vec<usize> = name_vec.iter().map(|f| f.len()).collect();
    for size in size_vec.iter() {
        println!("{size}");
    }

    for ele in Counter::new().into_iter() {
        println!("{ele}");
    }

    let vv: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .collect();

    for ele in vv.into_iter() {
        println!("{ele}");
    }
}
