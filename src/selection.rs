use crate::chromosome::Chromosome;
use crate::problem::Problem;

pub mod elitism;
pub mod random;
pub mod roulette;
pub mod tournament;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Selection {
    TournamentWithDuplicates,
    TournamentWithoutDuplicates,
    Roulette,
    Elitism,
    Random,
}

pub trait SelectionStrategy<T: Problem> {
    fn select(&self, population: &[Chromosome<T>], n: usize) -> Vec<Chromosome<T>>;
}
