use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::io;

#[derive(Clone, Debug)]
struct OneMax;
impl Problem for OneMax {
    type Fitness = usize;
    type Allele = u8;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        println!("{:?}", chromosome.genes);
        println!("Rate this chromosome from 0 to 10");
        let mut fitness = String::new();
        io::stdin()
            .read_line(&mut fitness)
            .expect("To take input from stdin");

        fitness.trim().parse().expect("To parse input to usize")
    }

    fn terminate(
        &self,
        population: &[Chromosome<Self>],
        _generation: u32,
        _temperature: f32,
    ) -> bool {
        population.iter().any(|c| c.genes.iter().sum::<u8>() == 5)
    }

    fn genotype() -> Vec<Self::Allele> {
        (0..5).map(|_| thread_rng().gen_range(0..=1)).collect_vec()
    }
}

fn main() {
    let genetic = GeneticBuilder::new()
        .with_fitness_target(5)
        .with_population_size(5)
        .with_problem(OneMax)
        .build();

    let result = genetic.run();
    println!("\n{result:?}");
}
