/*
    EX 1: Cálculo de idade

    Ao ser dado um ano de nascimento, calcule quanto anos a pessoa têm hoje (desconsiderar meses e dias)
*/

pub fn ex1_calculo_idade() {
    let ano_nascimento: u16 = 2000;
    let ano_atual: u16 = 2024;

    let mes_nascimento: u8 = 2;
    let mes_atual: u8 = 2;

    let dia_nascimento: u8 = 19;
    let dia_atual: u8 = 19;

    let mut idade: u8 = (ano_atual - ano_nascimento) as u8;
    if mes_nascimento > mes_atual {
        idade -= 1
    } else if dia_nascimento > dia_atual {
        idade -= 1
    };

    println!(
        "Ano de nascimento: {ano_nascimento}\nMês de nascimento: {mes_nascimento}\nDia de nascimento: {dia_nascimento}\n\tIdade atual:{idade}"
    );
}
