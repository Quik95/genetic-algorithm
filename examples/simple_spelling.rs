use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use itertools::Itertools;

use genetic_algorithm::selection::Selection;
use rand::{thread_rng, Rng};
use strsim::hamming;

const TARGET_WORD: &str = "supercalifragilisticexpialidocious";

#[derive(Eq, PartialEq, Hash, Default, Clone, Debug)]
struct Spelling;
impl Problem for Spelling {
    type Fitness = usize;
    type Allele = char;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        return TARGET_WORD.len()
            - hamming(TARGET_WORD, &chromosome.genes.iter().collect::<String>()).unwrap();
    }

    fn terminate(
        &self,
        population: &[Chromosome<Self>],
        _generation: u32,
        _temperature: f64,
    ) -> bool {
        population
            .iter()
            .any(|c| c.get_fitness() == TARGET_WORD.len())
    }

    fn genotype() -> Vec<Self::Allele> {
        (0..TARGET_WORD.len())
            .map(|_| thread_rng().gen_range('a'..='z'))
            .collect_vec()
    }
}

fn main() {
    let genetic = GeneticBuilder::new()
        .with_population_size(1000)
        .with_mutation_rate(0.1)
        .with_problem(Spelling)
        .with_selection_strategy(Selection::Roulette)
        .build();

    let res = genetic.run();
    println!("{res:?}");
    println!("\n{}", String::from_iter(res.genes));
}
