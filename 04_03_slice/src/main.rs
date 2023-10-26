



fn main() {

    let s = String::from("Hello world");

    let first = get_first_word(&s);

    let hello = &s[0..5];
    let hello = &s[..5];

    let world = &s[6..11];
    let world = &s[6..];

    let len = s.len();
    let slice = &s[0..len];
    let slice = &[..];

    let first = get_first_word_slice(&s);
    let first = better_get_first_word(&s);

}

fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn better_get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}