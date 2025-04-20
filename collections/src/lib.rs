use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

pub fn strings_lib() {
    let s1 = String::from("hello");
    let s2 = "World";
    let s3 = s1 + s2;

    println!("s3={}", s3);

    let mut empty = String::new();
    empty.push_str("hello");

    println!("empty={}", empty);

    // hello in Hindi
    let hindi = String::from("नमस्ते");
    for c in hindi.graphemes(false) {
        println!("{}", c);
    }
}

pub fn hash_lib() {
    let mut dic = HashMap::new();
    dic.insert("Blue", 1);

    // preferred, when keys won't be known at compile time
    let mut another_dic = HashMap::new();
    another_dic.insert(String::from("Blue"), 1);

    // update
    let key = String::from("Blue");
    another_dic.insert(key.clone(), 2); // overwrite

    // insert if not exists
    another_dic.entry(key).or_insert(1);

    let text = "Hello hello, nice to meet you, the world is your world, but it is my world as well";
    let mut counter: HashMap<String, i32> = HashMap::new();
    for word in text.split_whitespace() {
        counter
            .entry(word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    println!("counter={:?}", counter);

    let mut sorted: Vec<_> = counter.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("sorted={:?}", sorted);
}
