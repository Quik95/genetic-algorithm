use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

type Population = Vec<Vec<u8>>;
type Genome = Vec<u8>;

fn main() {
    let mut population = (0..100)
        .map(|_| {
            (0..1000)
                .map(|_| thread_rng().gen_range(0..=1))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    loop {
        let best: u32 = population
            .iter()
            .map(|g| g.iter().map(|&g| u32::from(g)).sum())
            .max()
            .unwrap();
        print!("\rCurrent best: {best}");

        if best == 1000 {
            break;
        }

        population = evaluate(population);
        let parents = selection(population);
        population = crossover(parents);
        population = mutate(population);
    }

    print!(
        "\nAnswer is: {}",
        population[0].iter().map(|&g| u32::from(g)).sum::<u32>()
    );
}

fn evaluate(p: Population) -> Population {
    p.into_iter()
        .map(|g| (g.iter().map(|&g| u32::from(g)).sum::<u32>(), g))
        .sorted_by_key(|x| x.0)
        .map(|(_sum, genome)| genome)
        .rev()
        .collect()
}

fn selection(p: Population) -> Vec<(Genome, Genome)> {
    p.into_iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple::<(Genome, Genome)>().unwrap())
        .collect()
}

fn crossover(g: Vec<(Genome, Genome)>) -> Population {
    let capacity = g.len() * 2;

    g.into_iter().fold(
        Vec::with_capacity(capacity),
        |mut acc, (mut father, mut mother)| {
            let cx_point = thread_rng().gen_range(0..=1000);

            let mut father_split = father.split_off(cx_point);
            let mut mother_split = mother.split_off(cx_point);

            father.append(&mut mother_split);
            mother.append(&mut father_split);

            acc.push(father);
            acc.push(mother);

            acc
        },
    )
}

fn mutate(p: Population) -> Population {
    p.into_iter()
        .map(|mut g| {
            if thread_rng().gen_range(0..=100) < 5 {
                g.shuffle(&mut thread_rng());
            }
            g
        })
        .collect_vec()
}
