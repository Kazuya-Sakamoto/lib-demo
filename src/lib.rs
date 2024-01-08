mod generator;

pub fn print_random_number() {
    let n = generator::gen_ran();
    print!("Random U8: {}", n)
}