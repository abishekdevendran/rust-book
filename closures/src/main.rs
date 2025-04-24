// fn expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     std::thread::sleep(std::time::Duration::from_secs(2));
//     intensity
// }

use std::{collections::HashMap, hash::Hash};

struct Cacher<T, U>
where
    U: Copy + Eq + Hash,
    T: Fn(U) -> U,
{
    pub calculation: T,
    memo: HashMap<U, U>
}

impl<T, U> Cacher<T, U>
where
    U: Copy + Eq + Hash,
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            memo: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.memo.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.memo.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", cache_result.value(intensity));
        println!("Next, do {} situps!", cache_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            cache_result.value(intensity)
        );
    }
}

fn main() {
    println!("Hello, world!");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // let simulated_user_specified_value = 30;
    // let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
