use crate::chromosome::Chromosome;
use num::cast::AsPrimitive;
use num::Num;
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

pub trait Problem: Clone + Debug {
    type Fitness: Num + PartialOrd + Display + Debug + Copy + Clone + AsPrimitive<f32>;
    type Allele: PartialOrd + Display + Debug + Clone;

    fn fitness(&self, chromosome: &Chromosome<Self>) -> Self::Fitness;
    fn terminate(&self, population: &[Chromosome<Self>], generation: u32, temperature: f32)
        -> bool;
    fn genotype() -> Vec<Self::Allele>;
}
