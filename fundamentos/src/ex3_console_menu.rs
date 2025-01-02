/*
    EX3:
        Construir um menu interativo para o usuário
        Utilizar loops e desvios condicionais

        Deve conter as seguintes ações
            - 1 -> Exibir uma mensagem
            - 2 -> Exibir a tabuada do 7
            - 3 -> Perguntar o nome do usuário
            - 4 -> Sair do programa
            - outro -> Mensagem de invalidez
            - letra -> Mensagem de invalidez
*/

use std::{io, thread::sleep, time};

pub fn menu() {
    // Loop principal
    loop {
        print!("{}[2J", 27 as char);

        menu_principal();

        println!("Qual sua opção?: ");
        let selecao: u8 = logica_cli_selecao();

        match selecao {
            1 => opcao_1(),
            2 => opcao_2(),
            3 => opcao_3(),
            4 => {
                opcao_4();
                break;
            }
            _ => invalidez(),
        };

        sleep(time::Duration::from_secs(3));
    }
}

fn menu_principal() {
    println!(
        "
\n=== MENU ===
    [1] - Mensagem especial
    [2] - Tabuada do 7
    [3] - Pergunta especial
    [4] - SAIR
    "
    )
}

fn logica_cli_selecao() -> u8 {
    let mut opcao_selecionada: String = String::new();
    io::stdin()
        .read_line(&mut opcao_selecionada)
        .expect("Inválido");

    match opcao_selecionada.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

fn opcao_1() {
    println!("\tFeliz Natal! 🦌🦌🎅🌲🌲");
}

fn opcao_2() {
    const MULTIPLICADOR: u8 = 7;
    for num in 1..=10 {
        println!("\t{} x {} = {}", num, MULTIPLICADOR, num * MULTIPLICADOR)
    }
}

fn opcao_3() {
    println!("\tQual seu nome?");
    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Desculpe, não entendi");

    println!("\tPrazer em lhe conhecer, {}!", name.trim())
}

fn opcao_4() {
    println!("Até mais!")
}

fn invalidez() {
    println!("Por gentileza, digite um número válido")
}
