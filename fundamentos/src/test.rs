pub fn calc_test(max_value: u32) {
    let mut result: u64 = 0;

    for num in 1..max_value + 1 {
        result += num as u64;
    }

    println!("Resultado final: {result}")
}
