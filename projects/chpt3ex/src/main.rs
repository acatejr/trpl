fn main() {
    let fahrenheit = 32.0;
    let celsius = f_to_c(fahrenheit);
    println!("{fahrenheit} fahrenheit is {celsius} celsius.");

    let fahrenheit = 100.0;
    let celsius = f_to_c(fahrenheit);
    println!("{fahrenheit} fahrenheit is {celsius} celsius.");

    let celsius = 0f32;
    let fahreneit = c_to_f(celsius);
    println!("{celsius} celsius is {fahreneit} fahreneit.");

    let celsius = 37f32;
    let fahreneit = c_to_f(celsius);
    println!("{celsius} celsius is {fahreneit} fahreneit.");

    let nth = 0;
    let f = fibo(nth);
    println!("{nth} fibo is {f}.");

    let nth = 1;
    let f = fibo(nth);
    println!("{nth} fibo is {f}.");

    for n in 0..19 {
        let f = fibo(n);
        println!("{n} fibo is {f}.");
    }

    twelve_days_of_christmas();

}

fn twelve_days_of_christmas() {

    let gifts = Vec::from([
        "a partridge in a pair tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ]);

    let days = Vec::from([
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelveth",
    ]);

    for i in 1..12 {
        let day = days[i-1];
        println!("On the {} of Christmas, my true love gave to me:", day);
        for v in &gifts[0..i+1] {
            println!("{}", v);
        }
        println!("\n");
    }
}

fn f_to_c(temp_f: f32) -> f32 {
    let temp_c: f32 = (temp_f - 32f32) * (5f32/9f32);
    return temp_c;
}

fn c_to_f(temp_c: f32) -> f32 {
    let temp_f: f32 = (temp_c * (9f32/5f32)) + 32f32;
    return temp_f;
}

fn fibo(n: i32) -> i32 {

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let f = fibo(n-1) + fibo(n-2);
    
    return f;
}