use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// fn   add_one_v1 (x: u32) -> u32 { x + 1 }
// let  add_one_v2 |x: u32| -> u32 { x + 1 };
// let  add_one_v3 |x|             { x + 1 };
// let  add_one_v4 |x|               x + 1  ;

struct Cacher<T: Fn(u32) -> u32> 
// where T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}


impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(value: T) -> Cacher<T> {
        Cacher { 
            calculation: value, 
            value: HashMap::new(),
        }
    }

    fn value(&mut self, key: u32) -> u32 {
        let opt = self.value.get(&key);
        let y: u32 = match self.value.get(&key) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(key);
                self.value.insert(key, v);
                v
            }
        };
        y
    }
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num: u32|  -> u32{
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // let cacher = Cacher::new(expensive_closure);

    let mut cacher = Cacher::new(|num: u32|  -> u32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity + 1));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}
