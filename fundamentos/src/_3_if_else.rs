/*
    -- CONTROLE DE FLUXO: DESVIOS CONDICIONAIS --

    Em Rust, desvios condicionais ocorrem através de 2 estruturas:
        [ if / else / else if ] --> Estruturas de decisão simples
        [ match ] --> Estrutura de asserção, similar ao Switch

    Todas as estruturas de decisão demandam o uso de escopo em bloco [ {} ], mesmo a versão inline

    Há 2 operadores lógicos úteis para o fluxod e desvios condicionais:
        AND = [ && ] --> Operador lógico "E": (a==b) && (b==c)
        OR = [ || ] --> Operador lógico "OU": (a>=b) || (b==5)

    Diferente de outras linguagens, o Rust não possui operador ternário, similar ao [ bool ? foo : bar ]

    O operador especial chamado [ match ] permite acionar ações com base no valor de entrada, similar ao Switch-case
*/

pub fn if_else() {
    const IS_TRUE: bool = true;

    // Desvio condicional simples
    if IS_TRUE {
        println!("The true is true")
    };

    // Desvio condicional com [ if else ]
    if IS_TRUE == false {
        println!("Ok")
    } else {
        println!("Ohh no!")
    };

    // Desvio condicional com [ else if ]
    if IS_TRUE == false {
        println!("🤯")
    } else if 3 > 2 {
        println!("🔑")
    } else {
        println!("🫢")
    }

    // Atribuição de variável através do [ let if ]
    let is_the_true_false: bool = if IS_TRUE == false { true } else { false };
    println!("The true is false? --> {}", is_the_true_false);

    // Operadores AND [ && ] e OR [ || ] para o [ if ]
    if (IS_TRUE) && (1 > 0) {
        println!("2 condições verdadeiras")
    }

    if (is_the_true_false) || (1 > 0) {
        println!("Somente 1 condição verdadeira")
    }

    // Desvio condicional através do [ match ]
    enum TheList {
        Item1,
        Item2,
        Item3,
    }

    let selected: TheList = TheList::Item2;

    match selected {
        TheList::Item1 => println!("❄️"),
        TheList::Item2 => println!("🔑"),
        _ => println!("Item desconhecido"),
    }
}
