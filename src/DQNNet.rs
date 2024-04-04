extern crate tch;
use tch::{nn, nn::Adam, nn::Module, nn::OptimizerConfig, Device, Tensor};

pub struct MyModel {
    n_input: i64,
    input: nn::Linear,
    linear1: nn::Linear,
    linear2: nn::Linear,
    output: nn::Linear,
}

impl MyModel {
    pub fn new(vs: &nn::Path, n_input: i64, n_dim: i64) -> MyModel {
        let n_input = n_input;
        let input = nn::linear(vs, n_input, n_dim, Default::default());
        let linear1 = nn::linear(vs, n_dim, n_dim, Default::default());
        let linear2 = nn::linear(vs, n_dim, n_dim, Default::default());
        let output = nn::linear(vs, n_dim, 4, Default::default());
        MyModel {
            n_input,
            input,
            linear1,
            linear2,
            output,
        }
    }

    pub fn forward(&self, xs: &Tensor) -> Tensor {
        let out1 = xs.view([-1, self.n_input]).apply(&self.input).relu();
        let out2 = out1.apply(&self.linear1).relu();
        let out3 = out2.apply(&self.linear2).relu();
        let out4 = out3.apply(&self.output);
        out4
        // let output_vector: Vec<f64> = out4.to_kind(tch::Kind::Double).into();
        // let output_array: [f64; 4] = output_vector[..4].try_into().unwrap();
        // output_array
    }
}
