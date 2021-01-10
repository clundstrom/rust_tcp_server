mod network_layer;
use log::{warn, error};
use std;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::net::TcpListener;
use network_layer::AbstractNetworkLayer;
use std::io::{Read};

fn main() {
    env_logger::init();

    // Get program entry arguments
    let args: Vec<String> = std::env::args().collect();

    // Pass immutable args
    let args: HashMap<&str, i32, RandomState> = sanitize_args(&args);

    // Insert argument values here
    let network: AbstractNetworkLayer = network_layer::AbstractNetworkLayer {
        bufsize: args["bufsize"],
        early_terminate: args["early_terminate"] as u8,
        port: args["port"] as u16,
        transfer_rate: args["transfer_rate"],
    };

    let listener: TcpListener = network.accept_connections();

    loop {
        for stream in listener.incoming() {

            let mut stream = stream.unwrap();
            println!("Accepted connection from {:?}", stream.peer_addr());

        }
    }
}

/// Sanitizes application arguments to meet requirements
///
/// Requirements: buff size, early_termination, port, transfer_rate
pub fn sanitize_args(args: &Vec<String>) -> HashMap<&str, i32, RandomState> {
    if args.len() > network_layer::MAX_ARGS as usize {
        error!("Args: {}, MAX: {}", args.len(), network_layer::MAX_ARGS);
        panic!("Too many arguments provided.")
    }

    // Verify values
    let mut map: HashMap<&str, i32> = HashMap::with_capacity(4);

    for i in 1..args.len() {
        let split: Vec<&str> = args[i].split("=").collect();

        let result = match split[1]
            .trim()
            .parse() {
            Ok(result) => result,
            Err(_) => {
                warn!("Parsing '{}', failed. Using default value {}.", split[1], network_layer::DEFAULTS[i - 1]);
                network_layer::DEFAULTS[i - 1]
            }
        };
        map.insert(split[0], result);
    }

    // Verify keys
    let keys = vec![&"bufsize", &"early_terminate", &"port", &"transfer_rate"];
    let mut mapped = vec![];

    for key in map.keys() {
        mapped.push(key);
    }
    // Sort keys
    mapped.sort();

    assert_eq!(keys, mapped, "Illegal argument mapping.");

    map
}