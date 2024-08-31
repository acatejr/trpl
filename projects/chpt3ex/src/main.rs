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
    let lyrics = "";

    for d in 1..13 {
        let mut day = String::new();

        if d == 1 {
            day = String::from("first");
        } else if d == 2 {
            day = String::from("second");
        }
        println!("On the {day} day of Christmas, \nmy true love gave to me\n");
    }
    println!("{lyrics}");
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