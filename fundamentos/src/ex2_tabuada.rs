/*
    EX2:
        Calcular os valores da tabuada de 1 a [X] utilizando os controladores de fluxo
*/

pub fn tabuada() {
    const NUM_MAXIMO: u16 = 20;
    const MULTIPLICADOR: u16 = 127;

    for num in 1..=NUM_MAXIMO {
        let result = (num * MULTIPLICADOR) as u16;
        println!("{num} x {MULTIPLICADOR} = {}", result);
    }
}
