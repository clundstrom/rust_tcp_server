mod network_layer;

//use log::{warn};
use std;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    sanitize_args(&args);
    // Insert argument values here
    let network = network_layer::AbstractNetworkLayer::new(1, 30, 8000, 1);

    network.schedule_task("Test")
}

/// Sanitizes application arguments to meet requirements
///
/// Requirements: Port, buff size.
pub fn sanitize_args(args: &Vec<String>) -> (i32, i32) {
    let mut res = vec![];

    for i in 1..args.len() {
        let split: Vec<&str> = args[i].split("=").collect();

        let result = split[1]
            .trim()
            .parse()
            .expect("Failed to parse argument. Not an integer.");

        res.insert(res.len(), result);
    }
    println!("{} {}", res[0], res[1]);
    (res[0], res[1])
}