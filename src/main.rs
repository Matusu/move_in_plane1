mod enviroment;
use enviroment::Action;
mod DQNNet;
use tch::{
    nn::{self, Adam, Module, Optimizer, OptimizerConfig},
    Device, Tensor,
};

fn optimize() -> Tensor {
    let device = tch::Device::cuda_if_available();
    let vs = nn::VarStore::new(device);
    let mut optimizer;
    let zaklad = Tensor::of_slice(&[0.0f32, 0.0, 0.0, 0.0]).to_device(device);
    let matrix: [[f32; 7]; 2] = [[0.0; 7]; 2];
    match nn::Adam::default().build(&vs, 1e-3) {
        Ok(opt) => optimizer = opt,
        _ => {
            println!("optimizer nenacten");
            return zaklad;
        }
    };
    let data: [f32; 7] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let net = DQNNet::MyModel::new(&vs.root(), 7, 256);
    let input_data = Tensor::of_slice(&data).to_device(device);
    // let loss = net
    //     .forward(&input_data)
    //     .cross_entropy_for_logits(&input_data);
    // optimizer.backward_step(&loss);
    let output = net.forward(&input_data);
    output
}

fn main() {
    let output = optimize();
    println!("{:?}", output);
}
