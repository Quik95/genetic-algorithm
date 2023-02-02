use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use genetic_algorithm::selection::Selection;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Stock {
    pub roi: u8,
    pub risk: u8,
}

impl Stock {
    pub const fn new(roi: u8, risk: u8) -> Self {
        Self { roi, risk }
    }
}

impl Display for Stock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ROI: {}, RISK: {})", self.roi, self.risk)
    }
}

const TARGET_FITNESS: isize = 180;

#[derive(Eq, PartialEq, Hash, Default, Debug, Clone)]
struct Portfolio {}
impl Problem for Portfolio {
    type Fitness = isize;
    type Allele = Stock;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        chromosome
            .genes
            .iter()
            .map(|c| c.roi as isize * 2 - c.risk as isize)
            .sum()
    }

    fn terminate(
        &self,
        population: &[Chromosome<Self>],
        _generation: u32,
        _temperature: f64,
    ) -> bool {
        population.iter().any(|c| c.get_fitness() > TARGET_FITNESS)
    }

    fn genotype() -> Vec<Self::Allele> {
        (0..10)
            .map(|_| Stock::new(thread_rng().gen_range(0..10), thread_rng().gen_range(0..10)))
            .collect_vec()
    }
}

fn main() {
    let g = GeneticBuilder::new()
        .with_population_size(20)
        .with_problem(Portfolio {})
        .with_selection_strategy(Selection::Elitism)
        .build();
    let best = g.run();
    println!("Fitness: {}", best.get_fitness());
    println!("Genes: {:?}", best.genes);
}
