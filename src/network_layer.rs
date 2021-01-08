use log::{warn};

pub struct AbstractNetworkLayer {
    transfer_rate: i32,
    bufsize: i32,
    early_terminate: u8,
    port: u16, // 0-65535
}

trait NetworkLayer {}

impl AbstractNetworkLayer {

    // Pseudo-constructor
    pub fn new(transfer_rate: i32, bufsize: i32, port: u16, early_terminate: u8) -> AbstractNetworkLayer {
        AbstractNetworkLayer { transfer_rate, bufsize, early_terminate, port }
    }

    ///
    /// Schedules Task to ThreadPool.
    ///
    pub fn schedule_task(&self, message: &str) -> () {
        println!("{}", &message);
    }
}
