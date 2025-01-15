mod _0_variaveis;
mod _1_shadowing;
mod _2_tipos_de_dados;
mod _3_if_else;
mod _4_loops;
mod _5_fn;
mod _6_memory;
mod ex1_calculo_idade;
mod ex2_tabuada;
mod ex3_console_menu;
mod ex_rust_1;
mod test;

fn main() {
    println!("\n-- VARIÁVEIS --");
    _0_variaveis::variaveis();

    println!("\n-- SHADOWING --");
    _1_shadowing::shadowing();

    println!("\n-- TIPOS DE DADOS --");
    _2_tipos_de_dados::tipos_de_dados();

    println!("\n-- CONTROLE DE FLUXO: DESVIOS CONDICIONAIS --");
    _3_if_else::if_else();

    println!("\n-- CONTROLE DE FLUXO: LAÇOS DE REPETIÇÃO --");
    _4_loops::loops();

    println!("\n-- FUNÇÕES --");
    _5_fn::funcoes();

    // println!("\n\n-- EXERCICIOS --");
    // println!("-- EX1:");
    // ex1_calculo_idade::ex1_calculo_idade();
    // println!("-- EX2:");
    // ex2_tabuada::tabuada();
    // println!("-- EX3:");
    // ex3_console_menu::menu();

    println!("-- EX-Rust 1:");
    //ex_rust_1::the_twelve_days_of_christmas();

    println!("\n-- TESTE --");
    test::calc_test(1_000_000);
}
