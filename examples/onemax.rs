use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use itertools::Itertools;
use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
struct OneMax;
impl Problem for OneMax {
    type Fitness = usize;
    type Allele = u8;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        chromosome.genes.iter().map(|&a| a as usize).sum()
    }

    fn terminate(
        &self,
        population: &[Chromosome<Self>],
        _generation: u32,
        _temperature: f32,
    ) -> bool {
        population.iter().any(|c| self.fitness(c) == 42)
    }

    fn genotype() -> Vec<Self::Allele> {
        (0..42).map(|_| thread_rng().gen_range(0..=1)).collect_vec()
    }
}

fn main() {
    let genetic = GeneticBuilder::new()
        .with_fitness_target(42)
        .with_population_size(100)
        .with_problem(OneMax)
        .build();

    let result = genetic.run();
    println!("\n{result:?}");
}
