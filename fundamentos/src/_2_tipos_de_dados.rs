/*
    -- TIPOS DE DADOS --

    Conceitos gerais sobre a tipagem em Rust:

        No Rust o sistema de tipagem atende a 2 propósitos:
            - Melhorar a otimização e segurança de memória ao estabelecer a cada momento o tipo de cada valor existente
            - Permitir uma análise mais precisa do código em tempo de compilação, rastreando potencais falhas de segurança enquanto o software é desenvolvido

        Para atender esses propósitos, o Rust estabelece a tipagem da seguinte forma:
            - Tipos escalares (primitivos) -> Focados em registrar dados em bits
            - Tipos compostos (objetos(?)) -> Focados na estruturação dos dados

        Ambas as formas buscam o mesmo objetivo: Estabelecer estruturas rigorosas para representação de dados no nível mais atômico da informação: As variáveis e instâncias
*/

pub fn tipos_de_dados() {
    /*
        - TIPOS ESCALARES
            Focados na representação primitiva de dados
    */

    // Valores inteiros
    let inteiro_8bits: i8 = 127;
    let inteiro_16bits: i16 = -32767;
    let inteiro_32bits: i32 = -2_147_483_647;
    let inteiro_64bits: i64 = 999_999_999_999_999_999;
    let inteiro_128bits: i128 = 1;
    let inteiro_bits_variaveis: isize = 0;

    println!(
        " - INTEIROS
        8 bits: {inteiro_8bits}
        16 bits: {inteiro_16bits}
        32 bits: {inteiro_32bits}
        64 bits: {inteiro_64bits}
        128 bits: {inteiro_128bits}
        bits variaveis: {inteiro_bits_variaveis}
    "
    );

    // Valores naturais (inteiros não-negativos)
    let natural_8bits: u8 = 123;
    let natural_16bits: u16 = 321;
    let natural_32bits: u32 = 999;
    let natural_64bits: u64 = 8888_7777_6666_555;
    let natural_128bits: u128 = 999;
    let natural_bits_variaveis: usize = 0;

    println!(
        " - NATURAIS
        8 bits: {natural_8bits}
        16 bits: {natural_16bits}
        32 bits: {natural_32bits}
        64 bits: {natural_64bits}
        128 bits: {natural_128bits}
        bits variaveis: {natural_bits_variaveis}
    "
    );

    // Valores com pontos flutuantes (decimais)
    let float_32bits: f32 = 0.1;
    let float_64bits: f64 = 9.999;

    println!(
        " - FLUTUANTES
        32 bits: {float_32bits}
        64 bits: {float_64bits}
    "
    );

    // Booleano
    let booleano: bool = true;
    println!(
        " - BOOLEANO
        booleano: {booleano}
    "
    );

    // Caractere
    let emoji: char = '🎅';
    println!(
        " - CARACTERE
        caracter: {emoji}
    "
    );

    /*
        TIPOS COMPOSTOS:
            - Focados em estruturar os dados
    */

    // Tuplas -> Agrupamento ordenado de valores de diferentes tipos
    let any_tuple: (i8, f32, char) = (3, 6.8, 'b');

    // Array -> Agrupamento fixo de valores de um mesmo tipo
    let any_array: [i8; 4] = [0, 3, 2, 5];

    // Slice -> Pedaço de um dado composo (string, array);
    let slice_array: &[i8] = &any_array[1..3];

    // Vetor -> Outra estrutura de dados similar a arrays
    let vetor: Vec<&[i8; 4]> = vec![&any_array];

    // Struct -> Estrutura de dados complexa, similar a uma classe sem métodos
    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
    }

    let stallone: User = User {
        name: String::from("Lopes"),
        age: 24,
    };

    // Enum -> Estrutura enumerada de dados
    #[derive(Debug)]
    enum Status {
        Ok,
        Error(String),
        Maybe,
    }

    let result: Status = Status::Ok;
    let result2: Status = Status::Error(String::from("It's not a problem"));
    let result3: Status = Status::Maybe;

    // Read the String field of the Error variant
    if let Status::Error(message) = &result2 {
        println!("Error message: {}", message);
    };

    // Trait -> Estrutura especial similar aos métodos de uma classe, cujo objetivo é ser uma espécie de "tipo abstrato" que define o método que uma [struct] poderá implementar
    trait Event {
        fn activate(&self) -> bool;
    }

    impl Event for User {
        fn activate(&self) -> bool {
            if self.age >= 18 {
                true
            } else {
                false
            }
        }
    }

    println!(
        " - COMPOSTOS
        Tupla: {:?}
        Array: {:?}
        slice: {:?}
        Vetor: {:?}
        Struct: {:?}
        Enum: {:?}, {:?} , {:?}
        Trait: {:?}
    ",
        any_tuple,
        any_array,
        slice_array,
        vetor,
        stallone,
        result,
        result2,
        result3,
        stallone.activate()
    );
}
