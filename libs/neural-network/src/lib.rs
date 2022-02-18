use std::iter::once;
use rand::{Rng, RngCore};


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}
#[derive(Clone, Copy, Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}
#[derive(Clone, Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}
#[derive(Clone, Debug)]
struct Neuron {
    bias: f32, 
    weights: Vec<f32>,
}

impl Network {
    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
       self.layers.iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers.windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            }).collect();

        Self {layers}
        
    }
    pub fn weights(&self) -> impl Iterator<Item = f32> + '_  {
        self.layers
        .iter()
        .flat_map(|layer| layer.neurons.iter())
        .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
        .cloned()
    }
    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::from_weights(
                    layers[0].neurons,
                    layers[1].neurons,
                    &mut weights,
                )
            })
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
    
}


impl Layer {
    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }
    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons)).collect();
        Self {neurons}
    }
    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

impl Neuron {
    fn propogate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let mut output = 0.0;
        for (&input, &weight) in inputs.iter().zip(&self.weights) {
            output += input * weight;
        }
        output += self.bias;
        output.max(0.0)
        
    }
    pub fn random(output_size : usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
        
        Self {bias, weights}
    }
    pub fn from_weights(
        output_neurons: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}