use crate::chromosome::Chromosome;
use crate::problem::Problem;
use crate::selection::SelectionStrategy;

use itertools::Itertools;
use num::cast::AsPrimitive;

use rand::{thread_rng, Rng};


#[derive(Default, Debug, Clone)]
pub struct RouletteSelection<T: Problem> {
    _problem: std::marker::PhantomData<T>,
}

impl<T: Problem> SelectionStrategy<T> for RouletteSelection<T> {
    fn select(&self, population: &[Chromosome<T>], n: usize) -> Vec<Chromosome<T>> {
        let sum_fitness = population
            .iter()
            .map(|c| c.get_fitness().as_())
            .sum::<f64>();
        (0..n)
            .map(|_| {
                let u = thread_rng().gen_range(0.0..=1.0) * sum_fitness;
                let mut acc = 0.0;
                for c in population {
                    acc += c.get_fitness().as_();
                    if acc > u {
                        return c.clone();
                    }
                }
                unreachable!("Roulette wheel selection failed");
            })
            .collect_vec()
    }
}
