use crate::chromosome::Chromosome;
use crate::problem::Problem;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::fmt::Debug;

pub struct GeneticAlgorithm<T: Problem> {
    population_size: u32,
    problem: T,
    fitness_target: Option<T::Fitness>,
    genotype: fn() -> Vec<T::Allele>,
    mutation_rate: f32,
}

impl<T: Problem> GeneticAlgorithm<T> {
    pub fn run(&self) -> Chromosome<T> {
        self.evolve()
    }

    fn evolve(&self) -> Chromosome<T> {
        let mut population = (0..self.population_size)
            .map(|_| Chromosome::new((self.genotype)()))
            .collect_vec();

        let mut i = 0;
        loop {
            i += 1;

            population = self.evaluate(population);

            let best = population.first().unwrap();
            let best_fitness = best.get_fitness();
            if i % 1000 == 0 {
                print!("Current best: {best_fitness:?}",);
                println!(" {}", best.genes.iter().map(ToString::to_string).join(""));
            }

            if self.problem.terminate(&population) {
                return best.clone();
            }

            let parents = Self::selection(population);
            population = Self::crossover(parents);
            population = Self::mutate(population, self.mutation_rate);
        }
    }

    fn evaluate(&self, p: Vec<Chromosome<T>>) -> Vec<Chromosome<T>> {
        p.into_iter()
            .map(|mut c| {
                c.set_fitness(self.problem.fitness(&c));
                c.age += 1;
                c
            })
            .sorted_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .rev()
            .collect()
    }

    fn selection(p: Vec<Chromosome<T>>) -> Vec<Option<(Chromosome<T>, Chromosome<T>)>> {
        p.into_iter()
            .chunks(2)
            .into_iter()
            .map(Itertools::collect_tuple)
            .collect()
    }

    fn crossover(g: Vec<Option<(Chromosome<T>, Chromosome<T>)>>) -> Vec<Chromosome<T>> {
        let length = g.len();

        g.into_iter()
            .fold(Vec::with_capacity(length * 2), |mut acc, t| {
                if let Some((mut father, mut mother)) = t {
                    let cx_point = thread_rng().gen_range(0..father.get_size());

                    let mut father_split = father.genes.split_off(cx_point);
                    let mut mother_split = mother.genes.split_off(cx_point);

                    father.genes.append(&mut mother_split);
                    mother.genes.append(&mut father_split);

                    acc.push(father);
                    acc.push(mother);
                }

                acc
            })
    }

    fn mutate(p: Vec<Chromosome<T>>, mutation_rate: f32) -> Vec<Chromosome<T>> {
        p.into_iter()
            .map(|mut g| {
                if rand::random::<f32>() <= mutation_rate {
                    g.genes.shuffle(&mut thread_rng());
                }
                g
            })
            .collect()
    }
}

pub struct GeneticBuilder<T: Problem> {
    fitness_target: Option<T::Fitness>,
    genotype: Option<fn() -> Vec<T::Allele>>,
    population_size: Option<u32>,
    problem: Option<T>,
    mutation_rate: Option<f32>,
}

impl<T: Problem> GeneticBuilder<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_fitness_target(mut self, fitness_target: T::Fitness) -> Self {
        self.fitness_target = Some(fitness_target);
        self
    }

    #[must_use]
    pub fn with_genotype(mut self, genotype: fn() -> Vec<T::Allele>) -> Self {
        self.genotype = Some(genotype);
        self
    }

    #[must_use]
    pub const fn with_population_size(mut self, population_size: u32) -> Self {
        self.population_size = Some(population_size);
        self
    }

    #[must_use]
    pub fn with_problem(mut self, problem: T) -> Self {
        self.problem = Some(problem);
        self
    }

    #[must_use]
    pub const fn with_mutation_rate(mut self, mutation_rate: f32) -> Self {
        self.mutation_rate = Some(mutation_rate);
        self
    }

    /// Build a GeneticAlgorithm
    ///
    /// # Panics
    /// Will panic if either `genotype` or `problem` are not set.
    #[must_use]
    pub fn build(self) -> GeneticAlgorithm<T> {
        assert!(self.genotype.is_some(), "genotype is required");
        assert!(self.problem.is_some(), "problem is required");

        GeneticAlgorithm {
            genotype: self.genotype.unwrap(),
            problem: self.problem.unwrap(),

            fitness_target: self.fitness_target,
            population_size: self.population_size.unwrap_or(100),
            mutation_rate: self.mutation_rate.unwrap_or(0.05),
        }
    }
}

impl<T: Problem> Default for GeneticBuilder<T> {
    fn default() -> Self {
        Self {
            fitness_target: None,
            genotype: None,
            population_size: None,
            problem: None,
            mutation_rate: None,
        }
    }
}
