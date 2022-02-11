use rand::Rng;


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32, 
    weights: Vec<f32>,
}

impl Network {
    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
       self.layers.iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }

    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers.windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            }).collect();

        Self {layers}
        
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
}