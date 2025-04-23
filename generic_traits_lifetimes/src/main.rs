use core::num;
use std::{fmt::Display, ops::Add};

fn main() {
    println!("Hello, world!");
    println!("{}", largest_in_list(vec![1, 2, 3, 4, 5]));
    let p1 = Point { x: 5, y: 10 };
    println!("p1.x={}", p1.x());
    println!("p1.distance_from_origin={}", p1.distance_from_origin());
    // let p2= Point { x: String::from("hello"), y: String::from("world") };
    // let p3= Point { x: 5.5, y: 2.2 };
    print_summary();
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    fn distance_from_origin(&self) -> i32 {
        (self.x.pow(2) + self.y.pow(2)).isqrt()
    }
}

fn largest_in_list<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) {
        println!("Read more from {}...", self.summarize_author());
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) {
        println!("{} - by {} ({})", self.headline, self.author, self.location);
    }
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}
impl Summary for Tweet {
    // fn summarize(&self) {
    //     println!("{}: {}", self.username, self.content);
    // }
    fn summarize_author(&self) -> String {
        self.username.clone()
    }
}

// fn notify(item: &impl Summary) {
fn notify<T: Summary>(item: &T) {
    item.summarize();
}

fn generate_summary_obj<T: Summary>() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Ice Hockey World Championship"),
        location: String::from("New York City, USA"),
        author: String::from("The Daily New York"),
        content: String::from("The New York Rangers won the Ice Hockey World Championship!"),
    }
}

fn print_summary() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Ice Hockey World Championship"),
        location: String::from("New York City, USA"),
        author: String::from("The Daily New York"),
        content: String::from("The New York Rangers won the Ice Hockey World Championship!"),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as a developer... learning to program is really fun. #rust",
        ),
        reply: false,
        retweet: false,
    };

    notify(&article);
    notify(&tweet);
}

// Crescendo
fn longest_with_announcement<'a, T>(a: &'a str, b: &'a str, ann: &'a T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if a.len() > b.len() { a } else { b }
}
