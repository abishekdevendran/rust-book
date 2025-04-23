use std::{fs::File, io::{self, Error, Read, Write}};

fn main() {
    // panic!("crash and burn");
    let f = File::open("hello.txt");
    // match f {
    //     Err(e) => println!("Error: {}", e),
    //     Ok(mut file) => {
    //         let mut s = String::new();
    //         match file.read_to_string(&mut s) {
    //             Err(e) => println!("Error: {}", e),
    //             Ok(_) => println!("{}", s),
    //         }
    //     }
    // }

    let mut f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(mut fc) => {
                    match fc.write_all(b"Hello, world!") {
                        Ok(_) => println!("File created!"),
                        Err(e) => panic!("Error: {}", e),
                    }
                    // reopen the file
                    match File::open("hello.txt") {
                        Ok(file) => file,
                        Err(e) => panic!("Error: {}", e),
                    }
                },
                Err(e) => panic!("Error: {}", e),
            },
            _ => panic!("Error: {}", e),
        }
    };

    let mut s = String::new();
    let s = match f.read_to_string(&mut s) {
        Ok(_) => s,
        Err(e) => panic!("Error: {}", e),
        
    };

    println!("{}", s);
    let ss = match read_file_contents() {
        Ok(s) => s,
        Err(e) => panic!("Error: {}", e),
    };
    guessing_game();
}

fn read_file_contents()->Result<String, Error>{

    // // let mut f = match File::open("hello.txt") {
    // //     Ok(file) => file,
    // //     Err(e) => return Err(e),
    // // };
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // // let s = match f.read_to_string(&mut s) {
    // //     Ok(_) => s,
    // //     Err(e) => return Err(e),
    // // };
    // f.read_to_string(&mut s)?;

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Guessing game struct pending
fn guessing_game() {
    let secret_number = 42;
    // let guess = String::new();
    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if !(1..=100).contains(&value) {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }

    // let mut guess = Guess::new(50);
    // read guess from user
    loop{
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);
        println!("You guessed: {}", guess.value());
        match guess.value().cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}