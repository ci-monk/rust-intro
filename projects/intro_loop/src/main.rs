fn check(){
    for i in 0..51 {
        if i == 0 { continue; }
        if i % 2 == 0 {
            println!("Par = {}", i);
        } else {
            println!("Ímpar = {}", i);
        }
    }
}

fn check_other(){
    for i in 0..21 {
        let value = if i % 2 == 0 {"Par"} else {"Ímpar"};
        println!("{} = {}", value, i);
    }
}

fn main() {
    for i in 0..10 {
        println!("Element = {}", i);
    }
    println!("");
    check();
    println!("");
    check_other();
}
