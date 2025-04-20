use collections::{hash_lib, strings_lib};

fn main() {
    let mut a1 = [1, 2, 3];
    // let mut v2: Vec<i32> = Vec::new();
    // for (i, el) in a1.iter().enumerate() {
    //     v2.push(el + i as i32);
    // }
    // println!("v2={:?}", v2);
    for (i, el) in a1.iter_mut().enumerate() {
        *el += i as i32;
    }
    println!("a1={:?}", a1);
    let a2 = [1, 2, 3, 4, 5, 6, 7];
    let a2: Vec<_> = a2
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, &el)| el)
        .collect();
    println!("a2={:?}", a2);
    // match a2.get(10) {
    //     Some(el) => println!("el={}", el),
    //     None => println!("none"),
    // }
    if let Some(el) = a2.get(10) {
        println!("el={}", el);
    } else {
        println!("none");
    }

    #[derive(Debug)]
    enum AllNumTypes {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let v_nums = vec![
        AllNumTypes::Int(1),
        AllNumTypes::Float(1.0),
        AllNumTypes::Text(String::from("hello")),
    ];
    println!("vNums={:?}", v_nums);
    match v_nums.get(1) {
        Some(x) => match x {
            AllNumTypes::Int(i) => println!("int={}", i),
            AllNumTypes::Float(f) => println!("float={}", f),
            AllNumTypes::Text(t) => println!("text={}", t),
        },
        _ => println!("none"),
    }
    strings_lib();
    hash_lib();
}
