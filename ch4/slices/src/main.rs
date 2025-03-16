fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    s
}

fn main() {
    let mut s = String::from("hello world");

    let first_word = first_word(&s);
    let second_word = second_word(&s);
    println!("{first_word}");
    println!("{second_word}");
    s.clear();
}
