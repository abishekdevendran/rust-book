use std::{fs::File, io::{Error, Read, Write}};

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