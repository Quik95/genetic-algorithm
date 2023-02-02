use crate::problem::Problem;
use std::fmt::Debug;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Chromosome<T: Problem + ?Sized> {
    pub genes: Vec<T::Allele>,
    fitness: Option<T::Fitness>,
    size: usize,
    pub age: u32,
}

impl<T: Problem + ?Sized> Chromosome<T> {
    #[must_use]
    pub fn new(genes: Vec<T::Allele>) -> Self {
        Self {
            size: genes.len(),
            fitness: None,
            age: 0,
            genes,
        }
    }

    pub fn get_fitness(&self) -> T::Fitness {
        self.fitness.unwrap()
    }

    pub fn set_fitness(&mut self, fitness: T::Fitness) {
        self.fitness = Some(fitness);
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}
