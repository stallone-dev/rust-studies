/*
    -- VARIÁVEIS E MUTABILIDADE --

    Variáveis são espaços, endereços no computador, onde valores podem ser guardados.

    No Rust, variaveis possuem uma forte característica chamada "mutabilidade", onde a variável é, por padrão, impossível de ser alterada após ser declarada

    Esse comportamento é possível de ser alterado, ao declarar explicitamente que a variável será mutável, através da keyword [mut]

    Há outros 2 tipos de variáveis:
        [const] --> Constantes que não permitem mutabilidade
        [static] --> Constantes que permitem mutabilidade no escopo especial unsafe{}
*/

pub fn variaveis() {
    // variável típica
    let x: i16 = 5;
    /*
        [ let ] --> keyword padrão para variáveis
        [ :i16 ] --> Tipagem estática
            Significa "inteiro de 16 bits"
        [ = 5 ] --> atribuição de valor

        Por padrão toda variável é IMUTÁVEL
            Não pode ser alterada nem em tipo nem em valor
    */

    println!("O valor de X é: {}", x);

    // Variável mutável (somente valor)
    let mut y: i16 = 5;
    /*
        [ mut ] --> keyword para tornar a variável mutável
            Mutabilidade somente de valor, para mudança de tipo é preciso
            funções específicas de transformação
    */
    println!("O valor de Y é: {}", y);
    y += 10;
    println!("O novo valor de Y é: {}", y);

    // Constante e tipo estátio
    const VALOR_CONSTANTE: i8 = 10;
    static VALOR_ESTATICO: i8 = 8;
    /*
       [const] e [static] --> keywords especias para variáveis constantes
           Variáveis constantes nunca podem ser alteradas

           É padronizado que as constantes sempre estarão em UPPER_CASE no Rust

        A diferença prática entre [const] e [static] é que a [const] jamais pode ser alterada após a inicialização dela no código, não permitindo o uso da keyword [mut]

        Já a [static] permite o uso de [mut], porém só permite a alteração do valor dentro de um escopo especial chamado [unsafe]
    */
    println!("O valor da constante é: {}", VALOR_CONSTANTE);
    println!("O valor estático é: {}", VALOR_ESTATICO);

    static mut VALOR_ESTATICO_MUTAVEL: i8 = 2;
    unsafe {
        println!("O valor estático mutável é: {}", VALOR_ESTATICO_MUTAVEL);
        VALOR_ESTATICO_MUTAVEL = 3;
        println!(
            "O novo valor estático mutável é: {}",
            VALOR_ESTATICO_MUTAVEL
        );
    }
}
