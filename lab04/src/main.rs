use std::{fs, io};
#[derive(Debug)]
enum MyError {
    NonASCII,
}
fn p1(path: &str) -> Result<(), io::Error> {
    let mut max_bytes = "";
    let mut max_chars = "";
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if line.chars().count() > max_chars.chars().count() {
            max_chars = line;
        }
        if line.len() > max_bytes.len() {
            max_bytes = line;
        }
    }
    println!("longest line byte-wise is: {}", max_bytes);
    println!("longest line character-wise is: {}", max_chars);
    Ok(())
}
fn main() {
    match p1("input.txt") {
        Ok(()) => (),
        Err(e) => println!("error occured: {}", e),
    }

    match p1("doesntexist.txt") {
        Ok(()) => (),
        Err(e) => println!("error occured: {}", e),
    }

    match rot13("Hello World") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("error: {:?}", e),
    }

    match rot13("Uryyb, Jbeyq!") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("error: {:?}", e),
    }

    match rot13("bună") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("error: {:?}", e),
    }

    match p4("hosts.txt") {
        Ok(()) => (),
        Err(e) => println!("error: {}", e),
    }

    match p4("doesntexist.txt") {
        Ok(()) => (),
        Err(e) => println!("error: {}", e),
    }

    match fs::read_to_string("p3.txt") {
        Ok(s) => println!("{}", p3(&s)),
        Err(e) => println!("error: {}", e),
    }

    match fs::read_to_string("doesntexist") {
        Ok(s) => println!("{}", p3(&s)),
        Err(e) => println!("error: {}", e),
    }
}

fn rot13(path: &str) -> Result<String, MyError> {
    let mut new_string = String::new();
    for c in path.chars() {
        if c as u32 > 127 {
            return Err(MyError::NonASCII);
        }
        if c as u8 >= b'a' && c as u8 <= b'z' {
            let pos = c as u8 - b'a';
            let mv = (pos + 13) % 26;
            let new_c = (mv + b'a') as char;
            new_string.push(new_c);
        } else if c as u8 >= b'A' && c as u8 <= b'Z' {
            let pos = c as u8 - b'A';
            let mv = (pos + 13) % 26;
            let new_c = (mv + b'A') as char;
            new_string.push(new_c);
        } else {
            new_string.push(c);
        }
    }
    Ok(new_string)
}

fn p3(text: &str) -> String {
    let mut new_text = String::new();
    for word in text.split_whitespace() {
        let full_word = match word {
            "pt" => "pentru",
            "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word,
        };
        if !new_text.is_empty() {
            new_text.push(' ');
        }
        new_text.push_str(full_word);
    }
    new_text
}

fn p4(path: &str) -> Result<(), io::Error> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if line.starts_with('#') {
            continue;
        }
        let mut columns = line.split_whitespace();
        if let (Some(ip), Some(name)) = (columns.next(), columns.next()) {
            println!("{} => {}", name, ip);
        }
    }
    Ok(())
}
