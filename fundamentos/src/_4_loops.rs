/*
    -- CONTROLE DE FLUXO: LOOPS --

    Junto aos desvios condicionais, existem estruturas de laço de repetição em Rust, sendo elas:
        [ loop ] --> Loop simples e incondicional, que permanecerá em execução até que se defina um [ break ] dentro do loop;
        [ while ] --> Loop condicional, onde a condição é determinada logo no início
        [ for ] --> Loop iteracional, voltado para percorrer itens em uma lista de elementos ou quantidade enumerável de iterações

    Uma boa prática recomendada para todos os loops é ter, declaradamente, um break-point definido
        No caso do [ loop ] --> adicionar alguma condicionante que acione o [ break ]
        No caso do [ while ] --> garantir que haja atualização da condicional
        No caso do [ for ] --> adicionar um limite máximo de iterações

    Todos os laços de repetição demandam o escopo de bloco [ {} ], e atualmente não existem outros laços além desses 3
        O laço tradicional "do while" foi eliminado pela equipe do Rust,aparentemente por causar muitos problemas de loop infinito
*/

pub fn loops() {
    // Loop simples incondicional
    let mut loop_counter: u8 = 1;
    loop {
        if loop_counter > 3 {
            break;
        }
        println!("\tLoop incondicional - {loop_counter}/3");
        loop_counter += 1;
    }

    // Loop while
    let mut while_counter: u8 = 1;
    while while_counter < 4 {
        println!("\tLoop While - {while_counter}/3");
        while_counter += 1;
    }

    // Loop for
    for for_counter in 1..=3 {
        println!("\tLoop for - {for_counter}/3");
    }

    // Outro exemplo de laço for
    let basic_limit: u8 = 9;
    const MAX_ITERATION: u8 = 5;

    println!("Loop for com limite máximo de iteração");
    for counter in 1..=basic_limit {
        if counter > MAX_ITERATION {
            break;
        };
        println!(
            "\tInteração: {counter}, limite normal:{basic_limit}, limite máximo: {MAX_ITERATION}"
        );
    }
}
