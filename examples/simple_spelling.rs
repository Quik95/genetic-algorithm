use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use itertools::Itertools;

use rand::{thread_rng, Rng};
use strsim::jaro_winkler;

const TARGET_WORD: &str = "supercalifragilisticexpialidocious";

#[derive(Clone, Debug)]
struct Spelling;
impl Problem for Spelling {
    type Fitness = f64;
    type Allele = char;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        return jaro_winkler(TARGET_WORD, &chromosome.genes.iter().collect::<String>());
    }

    fn terminate(
        &self,
        population: &[Chromosome<Self>],
        _generation: u32,
        _temperature: f32,
    ) -> bool {
        population.iter().any(|c| c.get_fitness() == 1.0)
    }

    fn genotype() -> Vec<Self::Allele> {
        (0..TARGET_WORD.len())
            .map(|_| thread_rng().gen_range('a'..='z'))
            .collect_vec()
    }
}

fn main() {
    let genetic = GeneticBuilder::new()
        .with_population_size(20)
        .with_problem(Spelling)
        .build();

    let res = genetic.run();
    println!("{res:?}");
    println!("\n{}", String::from_iter(res.genes));
}
