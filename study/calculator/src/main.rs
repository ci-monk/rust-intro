//Declare dependencies
use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .ok()
        .expect("Failed to Flush");
    stdin().read_line(input)
        .ok()
        .expect("Failed to Read");
}

fn main() {
    welcome_message();
    let operations = String::from("+-*/");

    loop {
        let mut primeiro = String::new();
        let mut segundo = String::new();
        let mut op = String::new();

        println!("\nQual o primeiro número? ");
        read(&mut primeiro);

        println!("Qual o segundo número? ");
        read(&mut segundo);

        println!("Qual a operação [+-*/]? ");
        read(&mut op);

        let primeiro: f64 = primeiro.trim().parse().unwrap();
        let segundo: f64 = segundo.trim().parse().unwrap();
        let op: char = op.trim().chars().next().unwrap();

        if !operations.contains(op) {
            println!("Operador inválido!\n");
            continue;
        }

        let resultado = match op {
            '+' => primeiro + segundo,
            '-' => primeiro - segundo,
            '*' => primeiro * segundo,
            '/' => primeiro / segundo,
            _ => panic!("Erro no operador!")
        };

        println!("O resultado de {} {} {} = {}\n", primeiro, op, segundo, resultado);

        let mut need = String::new();
        println!("Você deseja continuar [Y/N]?");
        read(&mut need);

        let need: char = need.to_ascii_uppercase().trim().chars().next().unwrap();
        match need {
            'Y' => continue,
            'N' => break,
            _ => panic!("Error ao continuar!")
        };
    }
}

fn welcome_message(){
    println!("---------------------------------------------------------\n");
    println!("- Bem vindo! Esse é o meu projeto de calculadora em Rust!\n");
    println!("---------------------------------------------------------");
}
