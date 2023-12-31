mod layer;
mod activation_functions;
use layer::Layer;

fn main() {
    let layer = Layer::new(3, 4);
    println!("{:?}", layer.getBias());
    println!("{:?}", layer.getWeights());
}
