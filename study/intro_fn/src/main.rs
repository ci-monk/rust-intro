fn sqr(x: i32) -> i32 {
    return x * x;
}

fn sqr_other(x: f64) -> f64 {
    return x * x;
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main() {
    println!("Hello, world!");
    let number = 3;
    let value = sqr(number);
    println!("Square is {}", value);

    let other = sqr_other(40.0);
    println!("Square Other is {}", other);

    let result = factorial(5);
    println!("Factorial {}", result);
}
