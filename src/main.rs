mod network_layer;
use log::{warn,error};
use std;
use std::panic::resume_unwind;


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
    if args.len() > network_layer::MAX_ARGS as usize{
        error!("Args: {}, MAX: {}", args.len(), network_layer::MAX_ARGS);
        panic!("Too many arguments provided.")
    }

    let mut res: Vec<i32> = vec![];


    for i in 1..args.len() {
        let split: Vec<&str> = args[i].split("=").collect();

        let mut result = match split[1]
            .trim()
            .parse() {
            Ok(result) => result,
            Err(_) => {
                warn!("Parsing failed. Using default {}", network_layer::DEFAULTS[i]);
                1
            }
        };
        res.insert(res.len(), result);
    }
    println!("{} {}", res[0], res[1]);
    (res[0], res[1])
}