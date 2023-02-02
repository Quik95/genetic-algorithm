use crate::chromosome::Chromosome;
use crate::problem::Problem;
use crate::selection::SelectionStrategy;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
pub struct TournamentWithDuplicates<T: Problem> {
    _problem: std::marker::PhantomData<T>,
}

impl<T: Problem> SelectionStrategy<T> for TournamentWithDuplicates<T> {
    fn select(&self, population: &[Chromosome<T>], n: usize) -> Vec<Chromosome<T>> {
        (0..n)
            .map(|_| {
                population
                    .choose_multiple(&mut thread_rng(), 2)
                    .max_by_key(|&c| c.get_fitness())
                    .unwrap()
                    .clone()
            })
            .collect_vec()
    }
}

#[derive(Default, Debug, Clone)]
pub struct TournamentWithoutDuplicates<T: Problem> {
    _problem: std::marker::PhantomData<T>,
}

impl<T: Problem> SelectionStrategy<T> for TournamentWithoutDuplicates<T> {
    fn select(&self, population: &[Chromosome<T>], n: usize) -> Vec<Chromosome<T>> {
        let mut selected = HashSet::new();

        while selected.len() < n {
            let winner = population
                .choose_multiple(&mut thread_rng(), 2)
                .max_by_key(|&c| c.get_fitness())
                .unwrap()
                .clone();

            selected.insert(winner);
        }

        selected.into_iter().collect_vec()
    }
}
