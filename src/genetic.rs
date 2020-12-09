extern crate rand;
extern crate entropy;

// use rand::{thread_rng, Rng};
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use rand::distributions::Alphanumeric;
use entropy::shannon_entropy;
use std::collections::BTreeMap;


#[test]
fn sample_fun() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}

pub fn generate_population(pop_size: i32) {

    let mut population = Vec::new(); 
    // const CHARSET: &[u8] = b"<{[(abcdefghijklmnopqrstuvwxyz _-+=/.,ABCDEFGHIJKLMNOPQRSTUVWXYZ)]}>";
    // const LEGNTH: usize = 30;

    // let mut rng = rand::thread_rng();

    // let mut x: String = String::from("");

    // for _ in 0..pop_size {
    //     let x: String = (0..LEGNTH)
    //     .map(|_| {
    //         let idx = rng.gen_range(0, CHARSET.len());
    //         CHARSET[idx] as char
    //     })
    //     .collect();   
    // }

    for _ in 0..pop_size {
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

        population.push(rand_string);
    }

    // for j in population.iter() {
    //     println!("{}", j);
    // }
    calculate_fitness(&mut population);
    // generate_parent(LEGNTH, CHARSET);
}

pub fn calculate_fitness(pop_vec: &mut Vec<String>) {
    let mut random: BTreeMap<&str, f32> = BTreeMap::new();

    let mut rng = rand::thread_rng();

    for _ in 1..=1000 {
        let num = rng.gen_range(0, pop_vec.len());
        let x: &String = &pop_vec[num];

        let entropy = shannon_entropy(x.as_bytes());

        random.insert(x, entropy);
    }

    // for (k, p) in random.iter() {
    // 	println!("{} \t=\t\t{}", k, p);
    // }

    generate_parent(&mut random);
}

pub fn generate_parent(arr: &mut BTreeMap<&str, f32>) {

	let mut parent: Vec<String> = Vec::new();
	let mut rng = rand::thread_rng();

    for (s, v) in arr.iter(){ 
    	if v>&4.8 {
    		let fit_parent: String = (&s).to_string();
    		parent.push(fit_parent);
    	}
	}

	// for i in parent.iter() {
	// 	println!("{}", i);
	// }

    let parent1 = parent.choose(&mut rng).unwrap().to_string();
    let parent2 = parent.choose(&mut rng).unwrap().to_string();
    // println!("{:?}", parent1);
    // println!("{:?}", parent2);

    crossover(parent1, parent2);
}

pub fn crossover(p1: String, p2: String) -> () {
    // let mut string = p1.replace_range(&mut self, range: (0..6).start_bound(), replace_with: &p2)
    let s1 = &p1[0..16];
    let s2 = &p2[15..30];

    let string = s1.to_string() + &s2.to_string();
    println!("{:?}", string);
    
    mutation(string);
}

fn mutation(string: String) {
    // let byte_string = string.as_bytes();
    // println!("{:?}", string.as_bytes());

    let mutated = string[15..30].to_string() + &string[0..16].to_string(); 

    println!("{:?}", mutated);
}