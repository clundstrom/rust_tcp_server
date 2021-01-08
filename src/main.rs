mod network_layer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Insert argument values here
    let network = network_layer::AbstractNetworkLayer;
    network.sanitize_args(&args);
    network.schedule_task("Test")

}
