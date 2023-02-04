use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::crossover::Crossover::Uniform;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use genetic_algorithm::selection::Selection::{
    Roulette, TournamentWithDuplicates, TournamentWithoutDuplicates,
};
use itertools::Itertools;
use once_cell::sync::{Lazy, OnceCell};
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;
use std::iter::Iterator;

static RAW_INSTANCE: &str = include_str!("/home/sebastian/Projects/Semestr III/Optymalizacja Kombinatoryczna/Projekt/BenchmarkRunner/data/m50.txt");
static PARALLEL_PROCESSORS: Lazy<u8> = Lazy::new(|| {
    RAW_INSTANCE
        .lines()
        .take(1)
        .map(|x| x.parse().unwrap())
        .next()
        .unwrap()
});
static JOBS: Lazy<Vec<u16>> = Lazy::new(|| {
    RAW_INSTANCE
        .lines()
        .skip(2)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect_vec()
});
static LOWER_BOUND: Lazy<i32> = Lazy::new(|| {
    (JOBS.iter().map(|&x| x as u32).sum::<u32>() as f32 / *PARALLEL_PROCESSORS as f32).ceil() as i32
});

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct PCmax;

impl Problem for PCmax {
    type Fitness = i32;
    type Allele = u8;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        *LOWER_BOUND
            - *chromosome
                .genes
                .iter()
                .zip(JOBS.iter())
                .fold(BTreeMap::<u8, u32>::new(), |mut acc, (&gene, &task)| {
                    let entry = acc.entry(gene).or_insert(0);
                    *entry += task as u32;
                    acc
                })
                .values()
                .max()
                .unwrap() as i32
    }

    fn terminate(
        &self,
        population: &[Chromosome<Self>],
        generation: u32,
        temperature: f64,
    ) -> bool {
        generation > 100_000 || population.iter().any(|x| x.get_fitness() == *LOWER_BOUND)
    }

    fn genotype() -> Vec<Self::Allele> {
        JOBS.iter()
            .map(|_| thread_rng().gen_range(0..*PARALLEL_PROCESSORS))
            .collect_vec()
    }
}

fn main() {
    let g = GeneticBuilder::new()
        .with_population_size(100)
        .with_mutation_rate(0.1)
        .with_crossover_strategy(Uniform(0.5))
        .with_selection_strategy(Roulette)
        .with_problem(PCmax)
        .build();

    let best = g.run();
    println!("{:?}", best);
}
