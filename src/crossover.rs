pub mod order_one;
pub mod single_point;
pub mod uniform;

use crate::chromosome::Chromosome;
use crate::problem::Problem;

pub enum Crossover {
    OrderOne,
    SinglePoint,
    Uniform(f64),
}

pub trait CrossoverStrategy<T: Problem> {
    fn crossover(
        &self,
        father: Chromosome<T>,
        mother: Chromosome<T>,
    ) -> (Chromosome<T>, Chromosome<T>);
}
