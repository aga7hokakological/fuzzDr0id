extern crate rand;

// use rand::{thread_rng, Rng};
use rand::Rng;

fn generate_population(pop_size: i32) {

    let mut population = Vec::new(); 
    const CHARSET: &[u8] = b"<{[(abcdefghijklmnopqrstuvwxyz _-+=/.,ABCDEFGHIJKLMNOPQRSTUVWXYZ)]}>";
    const LEGNTH: usize = 30;

    let mut rng = rand::thread_rng();

    // let mut x: String = String::from("");

    for _i in 0..pop_size {
        let x: String = (0..LEGNTH)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

        population.push(x);
    }

    // for j in population.iter() {
    //     println!("{}", j);
    // }
    calculate_fitness(&mut population);
    generate_parent(LEGNTH, CHARSET);
}

fn calculate_fitness(pop_vec: &mut Vec<String>) {

}

fn generate_parent(length: usize, charset: &[u8]) {
    let mut rng = rand::thread_rng();

    let parent1: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect();
    
    let parent2: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect();

    println!("{}", parent1);
    println!("{}", parent2);
}

fn main() {
    const POP_SIZE: i32 = 1000;
    // const CROSSOVER: f32 = 0.5;
    // const MUTATION: f32 = 0.13;
    // const MAX_GEN: i32 = 0;

    // let people: String = String::from("<{[(abcdefghijklmnopqrstuvwxyz _-+=/.,ABCDEFGHIJKLMNOPQRSTUVWXYZ)]}>");
    

    generate_population(POP_SIZE);
}