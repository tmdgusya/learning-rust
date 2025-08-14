
fn main() {
    let mut s = String::from("Hello, World!");
    let word = first_word(&s);
    println!("{}", word);

    s.clear();
    println!("{}, {}", s, word)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // find blank
            return i;
        }
    }

    s.len()
}
