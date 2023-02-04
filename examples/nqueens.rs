use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::crossover::Crossover::{OrderOne, Uniform};
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use genetic_algorithm::selection::Selection::{Elitism, TournamentWithDuplicates};
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct NQueens;
impl Problem for NQueens {
    type Fitness = i32;
    type Allele = u8;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        let diagonal_clashes = (0..8)
            .cartesian_product(0..8)
            .filter(|(i, j)| i != j)
            .filter(|(i, j)| {
                let dx = ((i - j) as i32).abs();
                let dy = (chromosome.genes[*i as usize] as i32
                    - chromosome.genes[*j as usize] as i32)
                    .abs();
                dx == dy
            })
            .count();
        let unique = chromosome.genes.iter().unique().count();
        unique as i32 - diagonal_clashes as i32
    }

    fn terminate(
        &self,
        population: &[Chromosome<Self>],
        generation: u32,
        temperature: f64,
    ) -> bool {
        population
            .iter()
            .max_by_key(|c| c.get_fitness())
            .unwrap()
            .get_fitness()
            == 8
    }

    fn genotype() -> Vec<Self::Allele> {
        let mut chromosome = (0..8).collect_vec();
        chromosome.shuffle(&mut thread_rng());

        chromosome
    }
}

fn main() {
    let g = GeneticBuilder::new()
        .with_population_size(100)
        .with_selection_strategy(Elitism)
        .with_crossover_strategy(Uniform)
        .with_problem(NQueens)
        .build();
    let best = g.run();
    println!("{:?}", best);
}
