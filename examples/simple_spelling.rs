use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use strsim::jaro_winkler;

const TARGET_WORD: &str = "supercalifragilisticexpialidocious";

#[derive(Clone, Debug)]
struct Spelling;
impl Problem for Spelling {
    type Fitness = f64;
    type Allele = char;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        jaro_winkler(TARGET_WORD, &chromosome.genes.iter().collect::<String>())
    }

    fn terminate(&self, population: &[Chromosome<Self>]) -> bool {
        population.iter().any(|c| c.get_fitness() == 1.0)
    }
}

fn main() {
    let genetic = GeneticBuilder::new()
        .with_genotype(|| {
            (0..TARGET_WORD.len())
                .map(|_| thread_rng().gen_range('a'..='z'))
                .collect_vec()
        })
        .with_population_size(20)
        .with_problem(Spelling)
        .build();

    let res = genetic.run();
    println!("{res:?}");
    println!("\n{}", String::from_iter(res.genes));
}
