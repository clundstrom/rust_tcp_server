mod network_layer;
use log::{info, warn, log_enabled, error, debug};
use std;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    env_logger::init();

    // Get program entry arguments
    let args: Vec<String> = std::env::args().collect();

    // Pass immutable args
    let args: HashMap<&str, i32, RandomState> = sanitize_args(&args);

    // Insert argument values here
    let network = network_layer::AbstractNetworkLayer::new(1, 30, 8000, 1);

    //network.schedule_task("Test")
}

/// Sanitizes application arguments to meet requirements
///
/// Requirements: Port, buff size.
pub fn sanitize_args(args: &Vec<String>) -> HashMap<&str, i32, RandomState> {
    if args.len() > network_layer::MAX_ARGS as usize {
        error!("Args: {}, MAX: {}", args.len(), network_layer::MAX_ARGS);
        panic!("Too many arguments provided.")
    }

    let mut map: HashMap<&str, i32> = HashMap::new();

    for i in 1..args.len() {
        let split: Vec<&str> = args[i].split("=").collect();

        let result = match split[1]
            .trim()
            .parse() {
            Ok(result) => result,
            Err(_) => {
                warn!("Parsing '{}', failed. Using default value {}.", split[1],network_layer::DEFAULTS[i - 1]);
                network_layer::DEFAULTS[i - 1]
            }
        };
        map.insert(split[0], result);
    }
    debug!("{:?}", map);
    map
}