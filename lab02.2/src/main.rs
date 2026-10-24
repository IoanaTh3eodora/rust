struct Note {
    title: String,
    body: String,
}
fn count_char(s: &str, c: char) -> usize {
    let mut i = 0;
    for ch in s.chars() {
        if ch == c {
            i += 1;
        }
    }
    i
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i] != b' ' {
        i += 1;
    }
    &s[..i]
}
fn reverse(s: &str) -> String {
    let mut r = String::new();
    for ch in s.chars() {
        r.insert(0, ch);
    }
    r
}
fn is_palindrome(s: &str) -> bool {
    reverse(s) == s
}
fn repeat_str(s: &str, n: usize) -> String {
    let mut s_ntimes = String::new();
    for _ in 0..n {
        s_ntimes.push_str(s);
    }
    s_ntimes
}
fn add_tag(s: &mut String, tag: &str) {
    s.push('[');
    s.push_str(tag);
    s.push(']');
}
fn censor(s: &mut String, c: char) {
    let mut x = String::new();
    for ch in s.chars() {
        if ch == c {
            x.push('*');
        } else {
            x.push(ch);
        }
    }
    *s = x;
}
fn shout(s: String) -> String {
    let mut s = s;
    s.make_ascii_uppercase();
    s
}
fn longer_of(a: String, b: String) -> String {
    if b.len() > a.len() { b } else { a }
}
fn make_note(title: &str, body: &str) -> Note {
    Note {
        title: String::from(title),
        body: String::from(body),
    }
}
fn word_count(note: &Note) -> usize {
    let mut count = 0;
    let mut a = false;
    for c in note.body.chars() {
        if c == ' ' {
            a = false;
        } else if !a {
            a = true;
            count += 1;
        }
    }
    count
}
fn append(note: &mut Note, text: &str) {
    if !note.body.is_empty() {
        note.body.push(' ');
    }
    note.body.push_str(text);
}
fn archive(note: Note) -> String {
    let mut x = String::from("[ARCHIVED] ");
    x.push_str(&note.title);
    x.push_str(": ");
    x.push_str(&note.body);
    x
}
fn merge(a: Note, b: &Note) -> Note {
    let mut a = a;
    a.title.push('+');
    a.title.push_str(&b.title);
    append(&mut a, &b.body);
    a
}

fn main() {
    // Part 1
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));

    // Part 2
    let mut s = String::from("hello world");
    add_tag(&mut s, "v2");
    println!("{s}");
    censor(&mut s, 'o');
    println!("{s}");

    // Part 3
    let loud = shout(s);
    println!("{loud}");
    let winner = longer_of(String::from("tiny"), loud);
    println!("{winner}");

    // Part 4
    let title = String::from("shopping");
    let mut n1 = make_note(&title, "milk");
    let n2 = make_note("ideas", "learn rust borrowing");
    println!("{title}");
    println!("{} word(s)", word_count(&n1));
    append(&mut n1, "eggs");
    append(&mut n1, "bread");
    println!("{}: {}", n1.title, n1.body);
    let merged = merge(n1, &n2);
    println!("{}: {}", merged.title, merged.body);
    println!("{} word(s)", word_count(&merged));
    println!("{}", archive(n2));
}
