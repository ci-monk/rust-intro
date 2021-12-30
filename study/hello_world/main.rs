/*
A função main() é especial: é sempre o primeiro código a
ser executado em cada programa Rust executável.

A primeira linha declara uma função chamada main que não
possui parâmetros e não possui retorno de nada. Se houver
parâmetros, eles ficarão entre parênteses.

Além disso, observe que o corpo da função está entre
colchetes {}. O Rust requer isso em todos os corpos
de função.

É um bom estilho colocar a chave de abertura na mesma
linha da declaração da função, adicionando um espaço no
meio.

Primeiro, o estilo Rust é recuar com quatro espaços,
não uma tabulação.

Em segundo lugar, println! se chama uma macro em Rust.
Se em vez disse, chamasse função, seria inserido como
println (sem o !).

Terceiro, passamos um ; para indicar que uma expressão
acabou e a próxima está pronta para começar. A maioria
das linhas do código Rust termina com um ponto e vírgula.

Se você está mais familiarizado com uma linguagem
dinâmica, como Ruby, Python ou JavaScript, pode não estar
acostumado a compilar e executar um programa como
etapas separadas. Rust é uma linguagem compilada
antecipadamente , o que significa que você pode compilar
um programa e fornecer o executável para outra pessoa, e
eles podem executá-lo mesmo sem ter o Rust instalado.

Apenas compilar com rustc é bom para programas simples,
mas conforme seu projeto cresce, você deseja gerenciar
todas as opções e facilitar o compartilhamento de seu
código. A seguir, apresentaremos a ferramenta Cargo, que
o ajudará a escrever programas Rust do mundo real.
*/

fn main(){
    println!("Meu primeiro programa em Rust!");
}
