use crate::chromosome::Chromosome;
use num::cast::AsPrimitive;

use std::fmt::{Debug, Display};
use std::hash::Hash;


pub trait Problem: Eq + PartialEq + Hash + Default + Clone + Debug {
    // TODO: Allow to represent Fitness with a floating point value
    // Currently floats are not supported because they don't implement Ord trait
    // Might rectify this in the future by using total_ordering
    type Fitness: Ord
        + PartialOrd
        + Eq
        + PartialEq
        + Hash
        + Display
        + Debug
        + Copy
        + Clone
        + AsPrimitive<f64>;
    type Allele: Eq + PartialEq + Hash + Display + Debug + Clone;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness;
    fn terminate(&self, population: &[Chromosome<Self>], generation: u32, temperature: f64)
        -> bool;
    fn genotype() -> Vec<Self::Allele>;
}
