use crate::chromosome::Chromosome;
use crate::crossover::{Crossover, CrossoverStrategy};
use crate::problem::Problem;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::marker::PhantomData;

#[derive(Clone, Default, Debug)]
pub struct OrderOne<T: Problem> {
    _problem: PhantomData<T>,
}

impl<T: Problem> CrossoverStrategy<T> for OrderOne<T> {
    fn crossover(
        &self,
        father: Chromosome<T>,
        mother: Chromosome<T>,
    ) -> (Chromosome<T>, Chromosome<T>) {
        let (mut start, mut end) = (
            thread_rng().gen_range(0..father.genes.len()),
            thread_rng().gen_range(0..father.genes.len()),
        );
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        let slice1 = &father.genes[start..end];
        let slice1_set: HashSet<_> = slice1.iter().cloned().collect();
        let p2_contrib = mother
            .genes
            .iter()
            .filter(|&x| !slice1_set.contains(x))
            .collect_vec();
        let (head1, tail1) = p2_contrib.split_at(start);

        let slice2 = &mother.genes[start..end];
        let slice2_set: HashSet<_> = slice2.iter().cloned().collect();
        let p1_contrib = father
            .genes
            .iter()
            .filter(|&x| !slice2_set.contains(x))
            .collect_vec();
        let (head2, tail2) = p1_contrib.split_at(start);

        let (c1, c2) = (
            head1
                .iter()
                .copied()
                .chain(slice1.iter())
                .chain(tail1.iter().copied())
                .cloned()
                .collect_vec(),
            head2
                .iter()
                .copied()
                .chain(slice2.iter())
                .chain(tail2.iter().copied())
                .cloned()
                .collect_vec(),
        );

        (Chromosome::new(c1), Chromosome::new(c2))
    }
}
