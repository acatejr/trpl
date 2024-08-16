fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    let spaces = "   ";
    println!("The value of spaces is: ={spaces}=");

    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let guess: u32 = "42".parse().expect("Not a number.");
    println!("The value of guess is: {guess}");

    let rem = 43 % 5;
    println!("The value of rem is: {rem}");

    let cat = '😺';
    println!("The value of cat is: {cat}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
