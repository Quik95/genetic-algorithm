use genetic_algorithm::genetic::{GeneticAlgorithm, GeneticBuilder};
use itertools::Itertools;
use rand::{thread_rng, Rng};

fn main() {
    let genetic = GeneticBuilder::new()
        .fitness_target(1000)
        .fitness_function(|g| g.iter().map(|&g| g as u32).sum())
        .genotype(|| (0..1000).map(|_| thread_rng().gen_range(0..=1)).collect())
        .population_size(100)
        .build();

    let result = genetic.run();
    println!("\n{result:?}");
}
