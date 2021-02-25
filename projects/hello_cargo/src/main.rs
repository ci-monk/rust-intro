use std::io;

/*
Para pegar informações externas e em seguida imprimir
o resultado como saída, precisaremos trazer a biblioteca
de entrada/saída io. Essa biblioteca vem como padrão,
conhecida como std.

A instrução let é usada para criar uma varíavel. No Rust
as variáveis são imutáveis por padrão:io

let foo = 5; // immutable
let mut bar = 5; // mutable

Ao usar a instrução mut, você declarará uma variável mutável.io
Essa variável mutável se chama guess e é igual a uma nova
instância de string.

String é um tipo de string fornecido pela biblioteca padrão
que é um bit de texto codificado em UTF-8 que pode
ser ampliado.

A sintáxe :: na ::new indica que new é uma função associda
ao tipo String. Algumas linguagens chamam isso de método estático.

Esta newfunção cria uma nova string vazia.

Você encontrará uma newfunção em muitos tipos,
porque é um nome comum para uma função que cria
um novo valor de algum tipo.

Para resumir, a linha let mut guess = String::new();
criou uma variável mutável que está atualmente associada
a uma nova instância vazia de a String.

Agora vamos chamar a função stdin do módulo io.

A função stdin retorna uma instância de std::io::Stdin,
que é um tipo que representa um identificador para a
entrada padrão do seu terminal.

A próxima parte do código, .read_line(&mut guess) chama o método
read_linem no identificador de entrada padrão para obter a entrada
do usuário. Também estamos passando um argumento para
read_line: &mut guess.

O trabalho do read_line é pegar tudo o que o usuário digitar
na entrada padrão e colocá-lo em uma string, de modo que essa
string seja um argumento. O argumento da string precisa ser
mutável para que o método possa alterar o conteúdo da string
adicionando a entrada do usuário.

O & indica que esse argumento é uma referência, o que fornece
uma maneira de permitir que várias partes do seu código acessem
um dado sem a necessidade de copiar esses dados para a memória
várias vezes.

As referências são um recurso complexo e uma das principais
vantagens do Rust é o quão seguro e fácil é usar as referências.

Por enquanto, tudo que você precisa saber é que,
como as variáveis, as referências são imutáveis ​​por padrão.
*/

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
