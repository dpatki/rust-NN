
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
    
}

impl Layer {
    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
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
}