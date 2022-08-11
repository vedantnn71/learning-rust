fn main() {
    let s = String::from("hello world s");

    let word = first_word(&s);

    println!("{word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}
