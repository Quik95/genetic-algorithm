use crate::chromosome::Chromosome;
use crate::problem::Problem;
use crate::selection::SelectionStrategy;
use itertools::Itertools;

#[derive(Default, Debug, Clone)]
pub struct ElitistSelection<T: Problem> {
    _problem: std::marker::PhantomData<T>,
}

impl<T: Problem> SelectionStrategy<T> for ElitistSelection<T> {
    fn select(&self, population: &[Chromosome<T>], n: usize) -> Vec<Chromosome<T>> {
        population.iter().take(n).cloned().collect_vec()
    }
}
