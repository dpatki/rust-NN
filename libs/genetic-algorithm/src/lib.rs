// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
#![feature(min_type_alias_impl_trait)]

use std::{ops::Index, iter::FromIterator};
use rand::{RngCore, prelude::SliceRandom, Rng};

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}
pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}


#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;


#[allow(clippy::len_without_is_empty)] 
impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }
    pub fn iter_nut(&mut self) -> impl  Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect()
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}
pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a[I]) -> &'a I 
    where
        I: Individual;
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}


impl CrossoverMethod for UniformCrossover {
    fn crossover (&self, rng: &mut dyn RngCore, 
        parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome {
            assert_eq!(parent_a.len(), parent_b.len());
            let parent_a = parent_a.iter();
            let parent_b = parent_b.iter();
            parent_a.zip(parent_b)
                .map(|(&a, &b)| if rng.gen_bool(0.5) {a} else {b})
                .collect()
        }
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0); 
        
        Self {chance, coeff}   
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_nut() {
            let sign = if rng.gen_bool(0.5) {-1.0} else {1.0};

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}
pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I 
    where 
        I: Individual,
    {
       population.choose_weighted(rng, |individual| individual.fitness())
            .expect("empty population")
    }
    
}
impl<S> GeneticAlgorithm<S> where S: SelectionMethod, {
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I] ) -> (Vec<I>, Statistics )
    where 
        I: Individual
    {
        assert!(!population.is_empty());
        let new_population = (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select( rng, population)
                    .chromosome();
                let parent_b = self.selection_method.select(rng, population)
                    .chromosome();
                let mut child = self.crossover_method
                    .crossover(rng, parent_a, parent_b);
                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            
            }).collect();
        
        let stats = Statistics::new(population);

        (new_population, stats)
    }

    
    pub fn new(selection_method: S, 
            crossover_method: impl CrossoverMethod + 'static,
            mutation_method: impl MutationMethod  +'static) -> Self {
        Self {selection_method, 
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method)}
    }
}

impl Statistics {
    fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as f32),
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }
}