use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genetic::GeneticBuilder;
use genetic_algorithm::problem::Problem;
use itertools::Itertools;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
struct Cargo {
    profits: Vec<u8>,
    weights: Vec<u8>,
    weight_limit: u16,
}

impl Cargo {
    fn get_weight(&self, mask: &[u8]) -> usize {
        mask.iter()
            .zip(self.weights.iter())
            .map(|(&a, &b)| a as usize * b as usize)
            .sum()
    }

    fn get_value(&self, mask: &[u8]) -> usize {
        mask.iter()
            .zip(self.profits.iter())
            .map(|(&a, &b)| a as usize * b as usize)
            .sum()
    }
}

impl Problem for Cargo {
    type Fitness = usize;
    type Allele = u8;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness {
        {
            if self.get_weight(&chromosome.genes) > self.weight_limit as usize {
                return 0;
            }
            self.get_value(&chromosome.genes)
        }
    }

    fn terminate(
        &self,
        _population: &[Chromosome<Self>],
        generation: u32,
        _temperature: f32,
    ) -> bool {
        generation == 1_000_000
    }

    fn genotype() -> Vec<Self::Allele> {
        (0..10).map(|_| thread_rng().gen_range(0..=1)).collect_vec()
    }
}

fn main() {
    let instance = Cargo {
        profits: vec![6, 5, 8, 9, 6, 7, 3, 1, 2, 6],
        weights: vec![10, 6, 8, 7, 10, 9, 7, 11, 6, 8],
        weight_limit: 40,
    };

    let g = GeneticBuilder::new()
        .with_population_size(50)
        .with_problem(instance.clone())
        .build();

    let res = g.run();
    println!("\nTotal value: {:?}", res.get_fitness());
    println!("Total weight: {:?}", instance.get_weight(&res.genes));
    println!("Genes: {:?}", res.genes);
}
