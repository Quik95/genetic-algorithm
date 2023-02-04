use crate::chromosome::Chromosome;
use crate::crossover::{Crossover, CrossoverStrategy};
use crate::problem::Problem;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::marker::PhantomData;

#[derive(Clone, Default, Debug)]
pub struct SinglePoint<T: Problem> {
    _problem: PhantomData<T>,
}

impl<T: Problem> CrossoverStrategy<T> for SinglePoint<T> {
    fn crossover(
        &self,
        mut father: Chromosome<T>,
        mut mother: Chromosome<T>,
    ) -> (Chromosome<T>, Chromosome<T>) {
        let cx_point = thread_rng().gen_range(0..father.get_size());

        let mut father_split = father.genes.split_off(cx_point);
        let mut mother_split = mother.genes.split_off(cx_point);

        father.genes.append(&mut mother_split);
        mother.genes.append(&mut father_split);

        (Chromosome::new(father.genes), Chromosome::new(mother.genes))
    }
}
