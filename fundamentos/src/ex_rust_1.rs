/*
    Exercício proposto pelo RUST (cap.3):

        Utilizar os conhecimetnos de variáveis, desvios condiconais e loops para formar a canção "The Twelve Days Of Christmas" no terminal console
*/

pub fn the_twelve_days_of_christmas() {
    const DAY_PHRASE: [(&str, &str); 12] = [
        ("primeiro", "E uma perdiz em uma pereira"),
        ("segundo", "Duas rolas"),
        ("terceiro", "Três galinhas francesas"),
        ("quarto", "Quatro pássaros chamando"),
        ("quinto", "Cinco anéis de ouro"),
        ("sexto", "Seis gansos-de postura"),
        ("sétimo", "Sete cisnes uma natação"),
        ("oitavo", "Oito empregadas domésticas uma ordenha"),
        ("nono", "Nove senhoras que dançam"),
        ("décimo", "Dez senhores a-salto"),
        ("décimo primeiro", "Onze gaiteiros que conduzem"),
        ("décimo segundo", "Doze bateristas tambores"),
    ];

    for (counter, (day, _phrase)) in DAY_PHRASE.iter().enumerate() {
        println!("\nNo {} dia de Natal", day);
        println!("Meu verdadeiro amor enviou para mim:");
        if counter == 0 {
            println!("\tA perdiz em uma árvore de pêra");
            continue;
        }
        for revert_counter in (0..=counter).rev() {
            println!("\t{}", DAY_PHRASE[revert_counter].1);
        }
    }
}
