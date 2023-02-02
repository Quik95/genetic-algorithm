use crate::chromosome::Chromosome;
use crate::problem::Problem;
use crate::selection::SelectionStrategy;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Default)]
pub struct RandomSelection<T: Problem> {
    _problem: std::marker::PhantomData<T>,
}

impl<T: Problem> SelectionStrategy<T> for RandomSelection<T> {
    fn select(&self, population: &[Chromosome<T>], n: usize) -> Vec<Chromosome<T>> {
        population
            .choose_multiple(&mut rand::thread_rng(), n)
            .cloned()
            .collect()
    }
}
