use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
Para obter a informação do usuário e, em seguida, imprimir o resultado como saída, precisamos
trazer a biblioteca io, de entrada e saída, para o escopo. A lib io vem da biblioteca padrçao
conhecida como std.

Rust vem com uma variedade de coisas em sua biblioteca padrão.
No entanto, se você tivesse que importar manualmente cada coisa que você usou, seria muito prolixo.
Mas importar muitas coisas que um programa nunca usa também não é bom. Um equilíbrio precisa ser alcançado.

O prelúdio é a lista de coisas que o Rust importa automaticamente para cada programa do Rust.
É mantido o mais pequeno possível e é focado nas coisas, particularmente nas características,
que são usadas em quase todos os programas Rust.

Prelúdios podem ser vistos como um padrão para tornar o uso de vários tipos mais conveniente.
Como tal, você encontrará outros prelúdios na biblioteca padrão, como std::io::prelude.

Por padrão o Rust traz apenas alguns tipos no âmbito de cada programa no prelúdio.
Se um tipo que você deseja usar não está no prelúdio, você deve trazer aquele tipo para
o escopo explicitamente com uma use declaração.

O uso da biblioteca std::io fornece vários recursos úteis, incluindo a capacidade de aceitar entradas do usuário.
*/
fn welcome(){
    println!("-------------------\n");
    println!("- Guess the number!\n");
    println!("-------------------");
}

fn main() {
    welcome();

    let numero_secreto = rand::thread_rng().gen_range(1..101);
    println!("O número secreto gerado foi: {}\n", numero_secreto);

    let mut numero = String::new();

    loop {
        numero.clear();

        println!("Por favor, informe seu número:");
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao ler a linha");

        let convert: u32 = match numero.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            },
        };

        println!("O número digitado foi: {}", numero);

        match convert.cmp(&numero_secreto) {
            Ordering::Less => println!("🥶 Muito pequeno!"),
            Ordering::Greater => println!("🥵 Muito grande!"),
            Ordering::Equal => {
                println!("😂 Ganhou!");
                break;
            }
        }
    }
}
