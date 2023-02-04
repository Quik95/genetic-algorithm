use crate::chromosome::Chromosome;
use crate::crossover::{Crossover, CrossoverStrategy};
use crate::problem::Problem;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::marker::PhantomData;

#[derive(Clone, Default, Debug)]
pub struct Uniform<T: Problem> {
    _problem: PhantomData<T>,
    crossover_rate: f64,
}

impl<T: Problem> Uniform<T> {
    pub fn new(crossover_rate: f64) -> Self {
        Self {
            _problem: PhantomData::default(),
            crossover_rate,
        }
    }
}

impl<T: Problem> CrossoverStrategy<T> for Uniform<T> {
    fn crossover(
        &self,
        father: Chromosome<T>,
        mother: Chromosome<T>,
    ) -> (Chromosome<T>, Chromosome<T>) {
        let (child1, child2) = father
            .genes
            .into_iter()
            .zip(mother.genes.into_iter())
            .map(|(f, m)| {
                if thread_rng().gen_bool(self.crossover_rate) {
                    (f, m)
                } else {
                    (m, f)
                }
            })
            .unzip();

        (Chromosome::new(child1), Chromosome::new(child2))
    }
}
