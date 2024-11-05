fn main() {
    let degs_c_in: f32 = 25.0;
    let degs_f_in: f32 = 80.0;
    let degs_f_out = temperature_converter(degs_c_in, "fahrenheit");
    let degs_c_out = temperature_converter(degs_f_in, "celsius");
    println!{"{degs_c_in}c is {degs_f_out}f"};
    println!("{degs_f_in}f is {degs_c_out}c");
    for n in 1..8 {
        let fib_n = fibonacci(n);
        println!("Fibonnaci number {n} is {fib_n}")
    }
}

// 1 - Celsius / Fahrenheit Conversion
fn _fahrenheit_to_celsius(degs_f: f32) -> f32 {
    let degs_c: f32 = (degs_f - 32.0) * (5.0/9.0);
    return degs_c
}

fn _celsius_to_fahrenheit(degs_c: f32) -> f32 {
    let degs_f: f32 = (degs_c * (9.0/5.0)) + 32.0;
    return degs_f
}

fn temperature_converter(degs: f32, unit: &str) -> f32 {
    match unit{
        "celsius" => _fahrenheit_to_celsius(degs),
        "fahrenheit" => _celsius_to_fahrenheit(degs),
        // Better error handling definitely needed in future
        _ => -999.0
    }
}

// 2 - Fibonacci
// Note for the future: A function can return itself, so this can be achieved
// by handling 0 and 1 explicitly and recursively calling for all other values
fn fibonacci(n: i32) -> i32 {
    let mut prev: i32 = 0;
    let mut val: i32 = 1;
    let mut temp: i32;
    if n == 1 {
        return 0
    } else if n == 2 {
        return 1
    } else {
        for _ in 0..n-2 {
            temp = val;
            val = val + prev;
            prev = temp;
        }
    }

    return val
}

// 3 - 12 days of Christmas
// Skipped due to the amount of typing involved!