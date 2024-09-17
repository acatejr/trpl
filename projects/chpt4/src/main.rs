fn main() {
    let mut s = String::from("Hello, world!");
    s.push_str(", word");
    println!("{s}");

    let s_len = calc_len(&s);
    println!("The string length is: {s_len}");
    println!("{s}");

    change(&mut s);
    println!("{s}");
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str(", world world");
}