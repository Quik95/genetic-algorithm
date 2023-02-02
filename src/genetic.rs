use crate::chromosome::Chromosome;
use crate::problem::Problem;
use crate::selection::elitism::ElitistSelection;
use crate::selection::random::RandomSelection;
use crate::selection::roulette::RouletteSelection;
use crate::selection::tournament::{TournamentWithDuplicates, TournamentWithoutDuplicates};
use crate::selection::{Selection, SelectionStrategy};
use itertools::Itertools;
use num::cast::AsPrimitive;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub struct GeneticAlgorithm<T: Problem> {
    population_size: u32,
    problem: T,
    fitness_target: Option<T::Fitness>,
    mutation_rate: f32,
    selection_rate: f32,
    selection_strategy: Box<dyn SelectionStrategy<T>>,
}

impl<T: Problem + 'static> GeneticAlgorithm<T> {
    pub fn run(&self) -> Chromosome<T> {
        self.evolve()
    }

    fn evolve(&self) -> Chromosome<T> {
        let mut population = (0..self.population_size)
            .map(|_| Chromosome::new(T::genotype()))
            .collect_vec();

        let mut generation = 0;
        let mut last_max_fitness = 0.0;
        let mut temperature: f64 = 0.0;

        let n = (population.len() as f32 * self.selection_rate).round() as usize;
        let n = if n % 2 == 0 { n } else { n + 1 };
        loop {
            population = self.evaluate(population);

            let best = population.first().unwrap();
            let best_fitness = best.get_fitness();

            temperature = 0.8 * (temperature + (best_fitness.as_() - last_max_fitness));

            if generation % 1000 == 0 {
                println!("Current best: {best_fitness:?} ({})", best.genes.len());
            }
            if generation % 10_000 == 0 {
                println!("{:?}", best.genes.iter().map(ToString::to_string).join(""));
            }

            if self.problem.terminate(&population, generation, temperature) {
                return best.clone();
            }

            let (parents, mut leftover) = self.selection(population, n);

            population = Self::crossover(parents);
            population.append(&mut leftover);
            while population.len() < self.population_size as usize {
                population.push(Chromosome::new(T::genotype()));
            }

            population = Self::mutate(population, self.mutation_rate);

            last_max_fitness = best_fitness.as_();
            generation += 1;
        }
    }

    fn evaluate(&self, p: Vec<Chromosome<T>>) -> Vec<Chromosome<T>> {
        p.into_iter()
            .map(|mut c| {
                c.set_fitness(self.problem.fitness(&c));
                c.age += 1;
                c
            })
            .sorted_by_key(Chromosome::get_fitness)
            .rev()
            .collect()
    }

    fn selection(
        &self,
        p: Vec<Chromosome<T>>,
        n: usize,
    ) -> (
        Vec<Option<(Chromosome<T>, Chromosome<T>)>>,
        Vec<Chromosome<T>>,
    ) {
        let parents = self.selection_strategy.select(&p, n);

        let population_set: HashSet<_> = p.into_iter().collect();
        let parents_set: HashSet<_> = parents.iter().cloned().collect();

        let leftover: Vec<_> = population_set
            .difference(&parents_set)
            .take(self.population_size as usize - parents.len())
            .cloned()
            .collect();

        let parents: Vec<_> = parents
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(Itertools::collect_tuple)
            .collect();

        (parents, leftover)
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
    problem: Option<T>,
    selection_strategy: Option<Box<dyn SelectionStrategy<T>>>,

    population_size: u32,
    mutation_rate: f32,
    selection_rate: f32,
}

impl<T: Problem + 'static> GeneticBuilder<T> {
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
    pub const fn with_population_size(mut self, population_size: u32) -> Self {
        self.population_size = population_size;
        self
    }

    #[must_use]
    pub fn with_problem(mut self, problem: T) -> Self {
        self.problem = Some(problem);
        self
    }

    #[must_use]
    pub const fn with_mutation_rate(mut self, mutation_rate: f32) -> Self {
        self.mutation_rate = mutation_rate;
        self
    }

    #[must_use]
    pub const fn with_selection_rate(mut self, selection_rate: f32) -> Self {
        self.selection_rate = selection_rate;
        self
    }

    #[must_use]
    #[allow(clippy::box_default)]
    pub fn with_selection_strategy(mut self, selection_strategy: Selection) -> Self {
        self.selection_strategy = match selection_strategy {
            Selection::TournamentWithDuplicates => {
                Some(Box::new(TournamentWithDuplicates::default()))
            }
            Selection::TournamentWithoutDuplicates => {
                Some(Box::new(TournamentWithoutDuplicates::default()))
            }
            Selection::Roulette => Some(Box::new(RouletteSelection::default())),
            Selection::Elitism => Some(Box::new(ElitistSelection::default())),
            Selection::Random => Some(Box::new(RandomSelection::default())),
        };

        self
    }

    /// Build a `GeneticAlgorithm`
    ///
    /// # Panics
    /// Will panic if either `problem` or `selection_strategy` is not set.
    #[must_use]
    pub fn build(self) -> GeneticAlgorithm<T> {
        assert!(self.problem.is_some(), "problem is required");
        assert!(
            self.selection_strategy.is_some(),
            "selection_strategy is required"
        );

        GeneticAlgorithm {
            problem: self.problem.unwrap(),
            selection_strategy: self.selection_strategy.unwrap(),

            fitness_target: self.fitness_target,
            population_size: self.population_size,
            mutation_rate: self.mutation_rate,
            selection_rate: self.selection_rate,
        }
    }
}

impl<T: Problem> Default for GeneticBuilder<T> {
    fn default() -> Self {
        Self {
            fitness_target: None,
            problem: None,
            selection_strategy: None,

            population_size: 100,
            mutation_rate: 0.05,
            selection_rate: 0.8,
        }
    }
}
