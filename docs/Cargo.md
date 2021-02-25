# Hello Cargo!

Cargo é o sistema de construção e gerenciador de pacotes do Rust. A maioria dos desenvolvedores usam essa ferramenta para gerenciar seus projetos Rust por que o Cargo lida com muitas tarefas para você, como construir seu código, baixar as bibliotecas das quais seu código depende e construir essas bibliotecas (chamamos de bibliotecas suas dependências de código).

Os programas Rust mais simples, como o que escrevemos até agora, não têm dependências. Conforme você escreve programas Rust mais complexos, você adicionará dependências e, se iniciar um projeto usando Cargo, adicionar dependências será muito mais fácil de fazer.

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

A primeira linha,, [package]é um título de seção que indica que as instruções a seguir estão configurando um pacote. À medida que adicionarmos mais informações a este arquivo, adicionaremos outras seções.

As próximas quatro linhas definem as informações de configuração de que o Cargo precisa para compilar seu programa: o nome, a versão, quem o escreveu e a edição do Rust a ser usada. O Cargo obtém seu nome e informações de e-mail de seu ambiente, então se essas informações não estiverem corretas, corrija as informações agora e salve o arquivo.

A última linha,, [dependencies]é o início de uma seção para você listar qualquer uma das dependências do seu projeto. No Rust, os pacotes de código são chamados de grades.
