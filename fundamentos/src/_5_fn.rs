/*
    -- FUNÇÕES --

    Em Rust, funções são definidas pela estrutura:
        [ fn nome_funcao () {} ]
    Onde:
        - [fn] => keyword de funções
        - [nome_funcao] => Nome dado à função
        - [()] => keyword de assinatura de parâmetros
        - [{}] => keywork para definição do bloco da função

    Funções atendem ao propósito de dividir as ações geradas pelo código em pequenas partes funcionais, reutilizáveis e personalizáveis, as funções.

    Funções não precisam de uma ordem específica de definição, porém, elas serão executadas na mesma ordem em que estiverem dentro da função especial "main", no entry-point do projeto


*/

pub fn funcoes() {
    println!("Numer: x = {}", mais_cinco(123))
}

fn mais_cinco(x: i8) -> i8 {
    if x > (127 - 5) {
        return 0;
    }
    x + 5
}
