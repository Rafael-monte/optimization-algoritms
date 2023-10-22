use rand::{Rng, seq::SliceRandom};

use crate::default_structures::BagItem;

#[derive(Clone)]
pub struct Chromossome {
    genes: Vec<bool>,
    mutation_rate: f32
}

const FIFTY_PERCENT: f32=0.5;
const MIN_MUTATION_RATE: f32=0_f32;
const MAX_MUTATION_RATE: f32=1_f32;
const DEFAULT_NUMBER_OF_GENERATIONS: i32=20;
const DEFAULT_MUTATION_RATE: f32=0.5;
const A_PAIR_OF_CHROMOSSOMES: i32=2;

impl Chromossome {
    pub fn new(gen_number: u32, mutation_rate: f32) -> Self {
        let mut genes: Vec<bool> = Vec::new();
        for _ in 0..gen_number {
            genes.push(rand::thread_rng().gen_bool(FIFTY_PERCENT as f64))
        }
        return Chromossome { genes, mutation_rate }
    }

    pub fn from_crossover(parents_genes: (&[bool], &[bool]), mutation_rate: f32) -> Self {
        let mut genes: Vec<bool> = Vec::new();
        genes.append(&mut parents_genes.0.to_vec());
        genes.append(&mut parents_genes.1.to_vec());
        let mut chromossome: Chromossome = Chromossome { genes, mutation_rate };
        chromossome.apply_mutation();
        return chromossome;
    }

    pub fn calc_fitness(&mut self, bag_items: &[BagItem], max_weight: f32) -> i32 {
        let mut current_weight: f32 = 0.0;
        let mut fitness: i32 = 0;
        for (idx, item) in bag_items.iter().enumerate() {
            if !self.genes[idx] {continue;}
            if current_weight + item.weight > max_weight {
                return 0;
            }
            fitness += item.utility as i32;
            current_weight += item.weight;
        }
        return fitness;
    }

    fn apply_mutation(&mut self) -> () {
        let need_mutate: bool = rand::thread_rng().gen_bool(self.mutation_rate as f64);
        if !need_mutate {return;}
        let number_of_bitshiftings: usize=rand::thread_rng().gen_range(0..self.genes.len());
        for _ in 0..=number_of_bitshiftings {
            let random_index: usize = rand::thread_rng().gen_range(0..self.genes.len());
            self.genes[random_index] = rand::thread_rng().gen_bool(FIFTY_PERCENT as f64)
        }
    }
}

fn create_initial_population(bag_items: &[BagItem], mutation_rate: f32) -> Vec<Chromossome> {
    if mutation_rate < MIN_MUTATION_RATE || mutation_rate > MAX_MUTATION_RATE {
        eprintln!("Invalid rate for mutation. Please insert something in interval: [0, 1]");
        panic!();
    }
    let mut population: Vec<Chromossome> = Vec::new();
    for _ in 0..bag_items.len() {
        population.push(Chromossome::new(bag_items.len() as u32, mutation_rate))
    }
    return population;
}

fn define_parents(chromossomes: &[Chromossome], bag_items: &[BagItem], max_weight: f32) -> Vec<Chromossome> {
    let number_of_parents: i32 = f32::floor(chromossomes.len() as f32 * FIFTY_PERCENT) as i32;
    let mut parents: Vec<Chromossome> = Vec::new();
    for _ in 0..=number_of_parents {
        let mut population_sample: Vec<Chromossome> = chromossomes
            .choose_multiple(&mut rand::thread_rng(), A_PAIR_OF_CHROMOSSOMES as usize)
            .cloned().collect();

        population_sample.sort_by(|a, b|{
            b.clone().calc_fitness(bag_items, max_weight).cmp(&a.clone().calc_fitness(bag_items, max_weight))   
        });
        parents.push(population_sample[0].clone());
    }
    return parents;
}

fn crossover(mom: Chromossome, dad: Chromossome, mutation_rate: f32) -> (Chromossome, Chromossome) {
    let break_point = rand::thread_rng().gen_range(1..mom.genes.len());
    let moms_genes = (&mom.genes[..break_point], &mom.genes[break_point..]);
    let dads_genes = (&dad.genes[..break_point], &dad.genes[break_point..]);
    let mut first_son = Chromossome::from_crossover((moms_genes.0, dads_genes.1), mutation_rate);
    let mut second_son = Chromossome::from_crossover((dads_genes.0, moms_genes.1), mutation_rate);
    first_son.apply_mutation();
    second_son.apply_mutation();
    return (first_son, second_son);
}

fn select_bag_items(best_chromossome: &Chromossome, bag_items: &[BagItem]) -> Vec<BagItem> {
    let mut selected_items: Vec<BagItem> = Vec::new();
    for (idx, gene) in best_chromossome.genes.iter().enumerate() {
        if *gene {
            selected_items.push(bag_items[idx]);
        }
    }
    return selected_items;
}

pub fn apply_genetic_algorithm(bag_items: &[BagItem], max_weight: f32, mutation_rate: Option<f32>, generations: Option<i32>) -> Vec<BagItem> {
    let mut current_population: Vec<Chromossome> = create_initial_population(bag_items,
         mutation_rate.unwrap_or(DEFAULT_MUTATION_RATE));

    for _ in 0..=generations.unwrap_or(DEFAULT_NUMBER_OF_GENERATIONS) {
        let mut new_population: Vec<Chromossome> = Vec::new();
        current_population.sort_by(|a, b| {
            b.clone().calc_fitness(bag_items, max_weight).cmp(&a.clone().calc_fitness(bag_items, max_weight))
        });

        let parents: Vec<Chromossome> = define_parents(&current_population, bag_items, max_weight);

        for idx in (0..parents.len()).step_by(A_PAIR_OF_CHROMOSSOMES as usize) {
            if idx + 1 == parents.len() {continue;}
            let (mom, dad) = (parents[idx].clone(), parents[idx+1].clone());
            let (f_son, s_son) = crossover(mom, dad, mutation_rate.unwrap_or(DEFAULT_MUTATION_RATE));
            let mut children = vec![f_son, s_son];
            new_population.append(&mut children)
        }
        current_population = new_population;
    }
    current_population.sort_by(|a, b| {
        b.clone().calc_fitness(bag_items, max_weight).cmp(&a.clone().calc_fitness(bag_items, max_weight))
    });

    return select_bag_items(current_population.first().unwrap(), bag_items);
}

