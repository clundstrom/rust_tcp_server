use log::{warn};
use std::collections::{HashMap};

pub struct AbstractNetworkLayer {
    transfer_rate: i32,
    bufsize: i32,
    early_terminate: bool,
    port: u16, // 0-65535
}

trait NetworkLayer {}

impl AbstractNetworkLayer {
    pub fn new(transfer_rate: i32, bufsize: i32, port: u16, early_terminate: bool) -> AbstractNetworkLayer {
        AbstractNetworkLayer { transfer_rate, bufsize, early_terminate, port }
    }

    ///
    /// Schedules Task to ThreadPool.
    ///
    pub fn schedule_task(&self, message: &str) -> () {
        println!("{}", &message);
    }


    ///
    /// Sanitizes application arguments to meet requirements
    ///
    /// Requirements: Port, buff size.
    ///
    pub fn sanitize_args(&self, args: &Vec<String>) {
        //let mut valid = HashMap::new();

        for i in 0..args.len() {
            let mut values: Vec<&str> = args[i].split(" ").collect();

            println!("Key: {}", args[i])
        }

        if args.len() < 1 {
            warn!("Using default port 8000");
        }
    }
}
