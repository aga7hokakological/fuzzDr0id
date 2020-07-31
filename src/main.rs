mod genetic;

fn main() {
    const POP_SIZE: i32 = 100000;
    const CROSSOVER: f32 = 0.8;
    const MUTATION: f32 = 0.13;
    // const MAX_GEN: i32 = 0;

    // let people: String = String::from("<{[(abcdefghijklmnopqrstuvwxyz _-+=/.,ABCDEFGHIJKLMNOPQRSTUVWXYZ)]}>");
    

    genetic::generate_population(POP_SIZE);
}