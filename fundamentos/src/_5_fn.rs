/*
    -- FUNÇÕES --

    Em Rust, funções são definidas pela estrutura:
        [ fn nome_funcao () {} -> type ]
    Onde:
        - [fn] => keyword de funções
        - [nome_funcao] => Nome dado à função
        - [()] => keyword de assinatura de parâmetros
        - [{}] => keyword para definição do bloco da função
        - [->] => Keyword para determiar a tipagem de retorno
        - [type] => Tipo de retorno da função

    Caso a função não retorne nada, o tipo padrão dela é [ -> () ].
    Funções atendem ao propósito de dividir as ações geradas pelo código em pequenas partes funcionais, reutilizáveis e personalizáveis, as funções.

    Funções não precisam de uma ordem específica de definição, porém, elas serão executadas na mesma ordem em que estiverem dentro da função especial "main", no entry-point do projeto

    Existem dois tipos de funções:
        - Declarativas => Executam algo, sem retornar valor
        - Expressivas => Executam ou não algo, sempre retornam um valor

    Dentro do escopo de uma função, existe a keyword [ return ] para realizar o encerramento da função mediante fluxos de dados, podendo assim alterar o resultado da função
*/

pub fn funcoes() {
    basic();

    let vetor: Vec<i32> = vec![10, 9, 8, 7, 5];
    println!("A soma do vetor é: {}", soma_vetor(&vetor));

    declarativa();
    expressiva();
    println!("Retorno: {}", com_return(4));
}

/// Descrição da função básica
fn basic() -> () {
    println!("Acionando função")
}

/// Função parametrizada para soma vetorial
fn soma_vetor(vetor: &Vec<i32>) -> i32 {
    vetor.iter().sum()
}

/// Exemplo função declarativa
fn declarativa() -> () {
    println!("Sem resultado")
}

/// Exemplo função expressiva
fn expressiva() -> u8 {
    println!("Com resultado");
    6
}

/// Exemplo 'return'
fn com_return(number: i32) -> i32 {
    if number % 2 == 0 {
        // Return antecipado para modificação do resultado com base no input
        return number * 2;
    }

    number % 2
}
