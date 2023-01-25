use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

pub type Genome = Vec<u8>;
pub type Chromosome = Vec<Genome>;

pub struct GeneticAlgorithm {
    genotype: fn() -> Genome,
    fitness_target: u32,
    fitness_function: fn(&Genome) -> u32,
    population_size: u32,
}

impl GeneticAlgorithm {
    pub fn run(&self) -> Genome {
        self.evolve()
    }
    fn evolve(&self) -> Genome {
        let mut population = (0..self.population_size)
            .map(|_| (self.genotype)())
            .collect_vec();
        loop {
            population = Self::evaluate(population, self.fitness_function);
            let best = population.first().unwrap();
            let best_fitness = (self.fitness_function)(best);
            print!("\rCurrent best: {best_fitness}");

            if best_fitness == self.fitness_target {
                return best.clone();
            }

            let parents = Self::selection(population);
            population = Self::crossover(parents);
            population = Self::mutate(population);
        }
    }

    fn evaluate(p: Chromosome, fitness: fn(&Genome) -> u32) -> Chromosome {
        p.into_iter()
            .map(|g| (fitness(&g), g))
            .sorted_by_key(|x| x.0)
            .map(|(_sum, genome)| genome)
            .rev()
            .collect()
    }

    fn selection(p: Chromosome) -> Vec<Option<(Genome, Genome)>> {
        p.into_iter()
            .chunks(2)
            .into_iter()
            .map(Itertools::collect_tuple)
            .collect()
    }

    fn crossover(g: Vec<Option<(Genome, Genome)>>) -> Chromosome {
        let length = g.len();

        g.into_iter()
            .fold(Vec::with_capacity(length * 2), |mut acc, t| {
                if let Some((mut father, mut mother)) = t {
                    let cx_point = thread_rng().gen_range(0..length);

                    let mut father_split = father.split_off(cx_point);
                    let mut mother_split = mother.split_off(cx_point);

                    father.append(&mut mother_split);
                    mother.append(&mut father_split);

                    acc.push(father);
                    acc.push(mother);
                }

                acc
            })
    }

    fn mutate(p: Chromosome) -> Chromosome {
        p.into_iter()
            .map(|mut g| {
                if rand::random::<f32>() <= 0.05 {
                    g.shuffle(&mut thread_rng());
                }
                g
            })
            .collect()
    }
}

#[derive(Default)]
pub struct GeneticBuilder {
    fitness_target: Option<u32>,
    fitness_function: Option<fn(&Genome) -> u32>,
    genotype: Option<fn() -> Genome>,
    population_size: Option<u32>,
}

impl GeneticBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn fitness_target(mut self, fitness_target: u32) -> Self {
        self.fitness_target = Some(fitness_target);
        self
    }

    #[must_use]
    pub fn fitness_function(mut self, fitness_function: fn(&Genome) -> u32) -> Self {
        self.fitness_function = Some(fitness_function);
        self
    }

    #[must_use]
    pub fn genotype(mut self, genotype: fn() -> Genome) -> Self {
        self.genotype = Some(genotype);
        self
    }

    #[must_use]
    pub const fn population_size(mut self, population_size: u32) -> Self {
        self.population_size = Some(population_size);
        self
    }

    /// Build a GeneticAlgorithm
    ///
    /// # Panics
    /// Will panic if either `genotype` or `fitness_function` are not set.
    #[must_use]
    pub fn build(self) -> GeneticAlgorithm {
        assert!(
            self.fitness_function.is_some(),
            "fitness_function is required"
        );
        assert!(self.genotype.is_some(), "genotype is required");

        GeneticAlgorithm {
            fitness_function: self.fitness_function.unwrap(),
            genotype: self.genotype.unwrap(),

            fitness_target: self.fitness_target.unwrap_or(1000),
            population_size: self.population_size.unwrap_or(100),
        }
    }
}
