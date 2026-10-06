fn isprime(n: u32) -> bool {
    if (n < 2) {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn cmmdc(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn coprime(a: u32, b: u32) -> bool {
    cmmdc(a, b) == 1
}

fn bottles(a: u32) -> String {
    if a == 0 {
        return String::from("No bottles");
    }
    if a == 1 {
        return String::from("1 bottle");
    }
    format!("{a} bottles")
}

fn sing() {
    let mut n = 99;
    while n >= 1 {
        println!("{} of beer on the wall,", bottles(n));
        println!("{} of beer.", bottles(n));
        println!("Take one down, pass it around,");
        println!("{} of beer on the wall.", bottles(n - 1));
        println!();
        n -= 1;
    }
}
fn main() {
    for n in 0..=100 {
        if isprime(n) {
            println!("{n}");
        }
    }
    println!();

    for a in 0..=100 {
        for b in a + 1..=100 {
            if coprime(a, b) {
                println!("{a}, {b}");
            }
        }
    }

    println!();
    sing();
}
