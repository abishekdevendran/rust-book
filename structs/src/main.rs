#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    println!("Hello, world!");
    let val = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("val={}", val);

    let v1 = plus_one(Some(5));
    let v2 = plus_one(None);
    if let Some(i) = v1 {
        println!("v1={}", i);
    } else {
        println!("v1 is None");
    }

    if let Some(i) = v2 {
        println!("v2={}", i);
    } else {
        println!("v2 is None");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}
