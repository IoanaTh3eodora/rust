fn add_chars_n(mut s: String, c: char, n: u32) -> String {
    let mut i = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
    s
}
fn add_chars_ref(s: &mut String, c: char, n: u32) {
    let mut i = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
}
fn add_space(s: &mut String, n: u32) {
    let mut i = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}
fn add_str(s: &mut String, t: &str) {
    s.push_str(t);
}
fn add_integer(s: &mut String, mut x: u32) {
    let mut tmp = String::new();
    let mut a = 0;
    while x > 0 {
        let c = ((x % 10) as u8 + b'0') as char;
        tmp.push(c);
        a += 1;
        x /= 10;
        if x > 0 && a == 3 {
            tmp.push('_');
            a = 0;
        }
    }
    for ch in tmp.chars().rev() {
        s.push(ch);
    }
}
fn add_float(s: &mut String, mut x: f64) {
    x += 0.0005;
    let int_x = x as u32;
    let mut tmp = String::new();
    let mut copy_int_x = int_x;
    if copy_int_x == 0 {
        tmp.push('0');
    }
    while copy_int_x > 0 {
        let c = ((copy_int_x % 10) as u8 + b'0') as char;
        tmp.push(c);
        copy_int_x /= 10;
    }
    for ch in tmp.chars().rev() {
        s.push(ch);
    }
    s.push('.');

    let mut frac_x = x - int_x as f64;
    for _ in 1..=3 {
        frac_x *= 10.0;
        let f = frac_x as u32;
        s.push((f as u8 + b'0') as char);
        frac_x = frac_x - f as f64;
    }
}
fn main() {
    let mut s0 = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s0 = add_chars_n(s0, c, 26 - i);

        i += 1;
    }

    println!("{}", s0);

    let mut s1 = String::from("");
    i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_ref(&mut s1, c, 26 - i);

        i += 1;
    }

    println!("{}", s1);

    let mut s2 = String::new();

    add_space(&mut s2, 40);
    add_str(&mut s2, "I");
    add_space(&mut s2, 1);
    add_str(&mut s2, "💚");
    add_str(&mut s2, "\n");
    add_space(&mut s2, 40);
    add_str(&mut s2, "RUST.");
    add_str(&mut s2, "\n");

    add_str(&mut s2, "\n");

    add_space(&mut s2, 4);
    add_str(&mut s2, "Most");
    add_space(&mut s2, 12);
    add_str(&mut s2, "crate");
    add_space(&mut s2, 6);
    add_integer(&mut s2, 306437968);
    add_space(&mut s2, 11);
    add_str(&mut s2, "and");
    add_space(&mut s2, 5);
    add_str(&mut s2, "lastest");
    add_space(&mut s2, 9);
    add_str(&mut s2, "is");
    add_str(&mut s2, "\n");

    add_space(&mut s2, 9);
    add_str(&mut s2, "downloaded");
    add_space(&mut s2, 8);
    add_str(&mut s2, "has");
    add_space(&mut s2, 13);
    add_str(&mut s2, "downloads");
    add_space(&mut s2, 5);
    add_str(&mut s2, "the");
    add_space(&mut s2, 9);
    add_str(&mut s2, "version");
    add_space(&mut s2, 4);
    add_float(&mut s2, 2.038);
    add_str(&mut s2, ".");
    add_str(&mut s2, "\n");

    println!("{s2}");
}

