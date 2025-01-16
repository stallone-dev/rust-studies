/*
    -- OWNERSHIP --

    Um dos problemas mais antigos e catastróficos da programação é a gestão de memória: Para executar algo, precisa-se guardar um pouco de memória em algum lugar, alguma variável, e manipulá-la pode gerar erros e falhas de segurança graves.

    Quando ocorre de um pedaço de memória ser corrompido, ou ser manipulado de maneira inesperada, surgem problemas como:
        - Data Races (acessos simultâneos em código concorrente)
        - Vazamentos de memória (manipulação incorreta dos ponteiros)
        - Dandling references (tentar acessar dados inválidos)
    Dentre outros ligados ao acesso ou manipulação indevida de memória

    Em linguagens de alto nível, como JavaScript, Python, Lua, existe a ferramenta "Garbage Collection", um tipo de rotina que vistoria e limpa a memória em tempo de execução, removendo o que não for mais usado pelo programa.

    A vantagem dele é diminuir a necessidade de se pensar sobre a gestão de memória. Ao preço de aumentar o consumo de memória e processamento para realizar essas limpezas periódicas.

    O RUST por sua vez opera com uma abordagem diferente: Um sistema de responsabilidade única a nível de memória, chamado "Ownership"
        - Em resumo, Ownership significa que cada dado, independente do tamanho, só poderá ter 1 único ponteiro apontado para ele a qualquer tempo do código.
        - Isso significa que se um certo dado for criado, ele pode ser somente
            - Referenciado, através do operador de referência [ & ]
            - Passado para outra variável, ao não usar operadores
        - Quando um dado é referenciado, pode ser usado múltiplas vezes dentro do escopo. Quando é passado, a variável anterior deixa de existir e se torna inválida, garantindo que não será usada erroneamente no futuro.

    O [ Owership ] se baseia em 3 regras:
        - 1. Cada valor em Rust possui uma variável que é dita seu owner (sua dona).
        - 2. Pode apenas haver um owner por vez.
        - 3. Quando o owner sai fora de escopo, o valor será destruído.

    Um determinado valor só existe em uma variável enquanto ela for a Owner, se a qualquer tempo for removida (passando para outra variável ou entrando em uma função), ela deixa de ser válida e retorna um erro de compilação.
*/

pub fn owner() -> () {
    println!("Caso 1: Mudando de Owner");
    caso_1();

    println!("\nCaso 2: Referenciando Owner");
    caso_2();

    println!("\nCaso3: Copiando dados profundamente");
    caso_3();
}

/// Exemplo de cenário onde o Owner é passado ao longo do programa
fn caso_1() -> () {
    // "value" não existe ainda
    let value = String::from("any"); // "value" se torna Owner de "any"

    let n_value = value; // "value" deixa de ser Owner
                         // "n_value" se torna o novo Owner do dado "any",
                         // "value" saí do escopo do programa e não é mais válida;

    exibir(n_value); // "n_value" passa o dado para dentro da função
                     // exibir(value);   // INVÁLIDA  ! -> "value" já saiu do escopo

    fn exibir(data: String) -> () {
        // "data" se torna Owner de "any"
        // Aqui é outro escopo com um tempo de vida próprio
        println!("O valor é: {}", data)
    } // "data" saí de escopo, valor "any" é liberado da memória e não há mais operações que assumam o Owner dele

    // Variáveis "value", "n_value", "data" e o próprio valor "any" já estão inválidos
}

/// Exemplo de caso onde o Owner nunca é passado, apenas referenciado
fn caso_2() -> () {
    // "value" não existe ainda
    let value = String::from("any"); // "value" se torna Owner de "any"
    println!("{value}"); // "value" empresta implicitamente

    let n_value = &value; // "n_value" aponta para "value"

    exibir(&value); // "value" empresta o dado p/função
    exibir(&n_value); // "n_value" empresta o dado p/função

    // Ambas as variáveis permanecem válidas e apontam para o mesmo dado "any"
    println!("{value} and {n_value}");

    fn exibir(data: &String) -> () {
        // "data" não se torna Owner, apenas acessa o local indicado pelo ponteiro `
        println!("O valor emprestado é: {data}")
    }
}

/// Exemplo de Owership com Deep Copy do dado
fn caso_3() -> () {
    let value = String::from("any");    // "value" se torna Owner de "any"
    let n_value = value.clone();        // "n_value" copia os dados de value
    
    // A nível de memória, existe agora na heap 2 alocações contendo o valor "any", ao invés de somente 1 como nos casos anteriores
    
    exibir(value);      // "value" passa o dado pra dentro da função
    exibir(n_value);    // "n_value" passa o dado para dentro da função
    
    // "value" e "n_value" saem de escopo
    
    fn exibir(data: String) -> () { // "data" se torna Owner do dado recebido
        println!("O valor é: {}", data)
    }
}
