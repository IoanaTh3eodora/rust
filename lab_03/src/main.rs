#[derive(Debug)]
enum Error {
    Overflow,
}
#[derive(Debug)]
enum CharError {
    NotAscii(char),
    NotDigit(char),
    NotBase16(char),
    NotLetter(char),
    NotPrint(char),
}
#[derive(Debug)]
enum CalcError {
    DivByZero,
    UnknownOperator(char),
    Overflow,
}
fn is_prime(x: u16) -> bool {
    let x = x as u32;
    if x == 0 || x == 1 {
        return false;
    }
    if x % 2 == 0 && x != 2 {
        return false;
    }
    let mut d = 3;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
fn next_prime(x: u16) -> Option<u16> {
    if x == u16::MAX {
        return None;
    }
    for y in (x + 1)..=u16::MAX {
        if is_prime(y) {
            return Some(y);
        }
    }
    None
}
fn checked_addition_u32(a: u32, b: u32) -> u32 {
    let result = a as u64 + b as u64;
    if result > u32::MAX as u64 {
        panic!("overflow issue: ");
    }
    result as u32
}

fn checked_mul_u32(a: u32, b: u32) -> u32 {
    let result = a as u64 * b as u64;
    if result > u32::MAX as u64 {
        panic!("overflow issue: ");
    }
    result as u32
}

fn checked_add_result(a: u32, b: u32) -> Result<u32, Error> {
    let result: u64 = a as u64 + b as u64;
    if result > u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok(result as u32)
}

fn checked_mul_result(a: u32, b: u32) -> Result<u32, Error> {
    let result: u64 = a as u64 * b as u64;
    if result > u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok(result as u32)
}

fn compute(a: u32, b: u32, c: u32) -> Result<u32, Error> {
    let mut result = checked_add_result(a, b)?;
    result = checked_mul_result(result, c)?;
    Ok(result)
}

fn to_uppercase(a: char) -> Result<char, CharError> {
    if a >= 'A' && a <= 'Z' {
        return Ok(a);
    }
    if a >= 'a' && a <= 'z' {
        return Ok(((a as u8) - 32) as char);
    }
    Err(CharError::NotLetter(a))
}

fn to_lowercase(a: char) -> Result<char, CharError> {
    if a >= 'a' && a <= 'z' {
        return Ok(a);
    }
    if a >= 'A' && a <= 'Z' {
        return Ok(((a as u8) + 32) as char);
    }
    Err(CharError::NotLetter(a))
}

fn print_char(a: char) -> Result<(), CharError> {
    if (a as u32) >= 32 && (a as u32) != 127 {
        println!("{}", a);
        return Ok(());
    }
    Err(CharError::NotPrint(a))
}

fn char_to_number(a: char) -> Result<u32, CharError> {
    if (a as u32) > 127 {
        return Err(CharError::NotAscii(a));
    }
    if a < '0' || a > '9' {
        return Err(CharError::NotDigit(a));
    }
    Ok((a as u32) - ('0' as u32))
}

fn char_to_number_hex(a: char) -> Result<u32, CharError> {
    if (a as u32) > 127 {
        return Err(CharError::NotAscii(a));
    }
    if a >= '0' && a <= '9' {
        return Ok((a as u32) - ('0' as u32));
    }
    if a >= 'a' && a <= 'f' {
        return Ok((a as u32) - ('a' as u32) + 10);
    }
    if a >= 'A' && a <= 'F' {
        return Ok((a as u32) - ('A' as u32) + 10);
    }
    Err(CharError::NotBase16(a))
}

fn print_error(e: CharError) {
    match e {
        CharError::NotAscii(c) => println!("{} is non-ASCII!", c),
        CharError::NotBase16(c) => println!("{} is not a valid hexadecimal digit", c),
        CharError::NotLetter(c) => println!("{} is not a letter!", c),
        CharError::NotPrint(c) => println!("{:?} is not printable!", c),
        CharError::NotDigit(c) => println!("{} is not a digit!", c),
    }
}

fn calculate(a: i32, op: char, b: i32) -> Result<i32, CalcError> {
    let a = a as i64;
    let b = b as i64;
    let result = match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0 {
                return Err(CalcError::DivByZero);
            }
            a / b
        }
        _ => return Err(CalcError::UnknownOperator(op)),
    };
    if result > i32::MAX as i64 || result < i32::MIN as i64 {
        return Err(CalcError::Overflow);
    }
    Ok(result as i32)
}

fn print_calc_error(e: CalcError) {
    match e {
        CalcError::DivByZero => println!("division by zero, undefined value!"),
        CalcError::UnknownOperator(c) => println!("{} is an unknown operator!", c),
        CalcError::Overflow => println!("overflow issue!"),
    }
}

fn main() {
    println!("P1");
    println!("{:?}", next_prime(10));
    println!("{:?}", next_prime(65521));
    let mut p = 0;
    while let Some(n) = next_prime(p) {
        println!("{}", n);
        p = n;
    }

    println!("P2");
    println!("{}", checked_addition_u32(5, 10));
    println!("{}", checked_mul_u32(1000, 1000));

    println!("P3");
    println!("{:?}", checked_add_result(5, 10));
    println!("{:?}", checked_add_result(u32::MAX, 1));
    println!("{:?}", checked_mul_result(1000, 1000));
    println!("{:?}", checked_mul_result(u32::MAX, 2));

    println!("other functions");
    println!("{:?}", compute(2, 3, 4));
    println!("{:?}", compute(u32::MAX, 1, 2));
    println!("{:?}", compute(100000, 0, 100000));

    println!("P4");
    match to_uppercase('a') {
        Ok(c) => println!("{}", c),
        Err(e) => print_error(e),
    }
    match to_uppercase('3') {
        Ok(c) => println!("{}", c),
        Err(e) => print_error(e),
    }
    match to_lowercase('B') {
        Ok(c) => println!("{}", c),
        Err(e) => print_error(e),
    }
    match to_lowercase('#') {
        Ok(c) => println!("{}", c),
        Err(e) => print_error(e),
    }
    match print_char('x') {
        Ok(()) => {}
        Err(e) => print_error(e),
    }
    match print_char('\n') {
        Ok(()) => {}
        Err(e) => print_error(e),
    }
    match char_to_number('7') {
        Ok(n) => println!("{}", n),
        Err(e) => print_error(e),
    }
    match char_to_number('x') {
        Ok(n) => println!("{}", n),
        Err(e) => print_error(e),
    }
    match char_to_number('ș') {
        Ok(n) => println!("{}", n),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('f') {
        Ok(n) => println!("{}", n),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('C') {
        Ok(n) => println!("{}", n),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('g') {
        Ok(n) => println!("{}", n),
        Err(e) => print_error(e),
    }

    println!("P5");
    match calculate(10, '+', 5) {
        Ok(r) => println!("{}", r),
        Err(e) => print_calc_error(e),
    }
    match calculate(3, '-', 8) {
        Ok(r) => println!("{}", r),
        Err(e) => print_calc_error(e),
    }
    match calculate(5, '/', 0) {
        Ok(r) => println!("{}", r),
        Err(e) => print_calc_error(e),
    }
    match calculate(1, '%', 2) {
        Ok(r) => println!("{}", r),
        Err(e) => print_calc_error(e),
    }
    match calculate(i32::MAX, '+', 1) {
        Ok(r) => println!("{}", r),
        Err(e) => print_calc_error(e),
    }
}
