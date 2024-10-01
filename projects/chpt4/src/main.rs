fn main() {
    let word = String::from("abc defg");
    let fw = first_word(&word);
    println!("{fw}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// fn calc_len(s: &String) -> usize {
//     s.len()
// }

// fn change(some_str: &mut String) {
//     some_str.push_str(", world world");
// }