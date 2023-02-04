use crate::chromosome::Chromosome;
use crate::problem::Problem;
use crate::selection::SelectionStrategy;

use itertools::Itertools;
use num::cast::AsPrimitive;

use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Default, Debug, Clone)]
pub struct RouletteSelection<T: Problem> {
    _problem: std::marker::PhantomData<T>,
}

impl<T: Problem> SelectionStrategy<T> for RouletteSelection<T> {
    fn select(&self, population: &[Chromosome<T>], n: usize) -> Vec<Chromosome<T>> {
        (0..n)
            .map(|_| {
                population
                    .choose_weighted(&mut thread_rng(), |c| c.get_fitness().as_())
                    .unwrap()
                    .clone()
            })
            .collect_vec()
    }
}
