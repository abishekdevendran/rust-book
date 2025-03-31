fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The sum of 2 and 3 is {}", add(2, 3));
    for i in (3..=10).rev() {
        println!("{i}");
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
