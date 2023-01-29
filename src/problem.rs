use crate::chromosome::Chromosome;
use std::fmt::{Debug, Display};

pub trait Problem: Clone + Debug {
    type Fitness: PartialOrd + Display + Debug + Copy + Clone;
    type Allele: PartialOrd + Display + Debug + Clone;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness;
    fn terminate(&self, population: &[Chromosome<Self>]) -> bool;
}
