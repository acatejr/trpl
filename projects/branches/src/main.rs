fn main() {
    let number = 3;
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    if number != 0 {
        println!("number is {number}");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");
}
