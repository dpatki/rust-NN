use crate::*;

#[derive(Debug)]
pub struct World {
    crate animals: Vec<Animal>,
    crate foodstuffs: Vec<Food>
}
impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(
            |_| Animal::random(rng)
        ).collect();

        let foodstuffs = (0..60).map(
            |_| Food::random(rng)
        ).collect();

        Self {animals, foodstuffs}
    }
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }
    pub fn foods(&self) -> &[Food] {
        &self.foodstuffs
    }
}
