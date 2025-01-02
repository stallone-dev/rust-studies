/*
    -- Shadowing e sobrescrita de tipos --

    No Rust, é possível realizar a reatribuição de valores simultâneo a mudança de tipagem ao se aplicar o [shadowing]:

    [Shadowin] é a redeclaração da variável, onde neste processo é possível modificar completamente o propósito da variável
    
    Exemplo:
        let x: u8 = 1;
        let x: i8: -1; (mudança de tipagem, preservação do nome da variável)

        let x: u8 = 10;
        let x: i8 = x; (mudança de tipagem com preservação do valor e nome)

        let x: u8 = 11;
        let x: String = x.to_string(); 
            Aqui há uma mudança drástica de tipagem e valor, passando a ser uma string que representa o valor original de X
*/

pub fn shadowing() {
    let z: i16 = 1024;

    println!("O valor de z é: {} - Endereço: {:p}", z, &z);
    /*
        Além do valor da variável, existe o endereço de memória dela, representado pela propriedade especial [&z], que é visualizada pelo placeholder especial [{:p}] da macro println!

        O que isso está mostrando é o endereço de memória onde está o valor da variável
    */

    let mut z_mut: i16 = 10;
    println!("O valor de z_mut é: {} - Endereço: {:p}", z_mut, &z_mut);
    z_mut += 10;
    println!(
        "O novo valor de z_mut é: {} - Endereço: {:p}",
        z_mut, &z_mut
    );
    /*
        Quando se declara uma variável mutável, e se altera o valor dela, o endereço da memória não é alterado, apenas o valor contido dentro dele
    */

    let z_shadow: i8 = 3;
    println!(
        "O valor de z_shadow é: {} - Endereço: {:p}",
        z_shadow, &z_shadow
    );
    let z_shadow: i16 = 8000;
    println!(
        "O novo valor de z_shadow é: {} - Endereço: {:p}",
        z_shadow, &z_shadow
    );
    /*
        Ao redeclarar a variável, é possível alterar o tipo e valor dela, uma vez que também se é alterado o endereço de memória dela, através do [shadowing]
    */
}
