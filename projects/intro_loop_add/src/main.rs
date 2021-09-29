fn sum(){
    let mut value = 0.0;
    for i in 0..10 {
        value += i as f64;
        println!("Valor atual: {}", value);
    }
    println!("O valor final é: {}", value);
}

fn main() {
    println!("Hello, world!");
    sum();
}
