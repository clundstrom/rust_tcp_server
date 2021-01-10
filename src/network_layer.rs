// Rate,buf,port,early_term
pub const MAX_ARGS: i32 = 5;
pub const DEFAULTS: [i32; 4] = [100, 64, 8000, 0];

pub struct AbstractNetworkLayer {
    transfer_rate: i32,
    bufsize: i32,
    port: u16,
    // 0-65535
    early_terminate: u8,
}

impl AbstractNetworkLayer {
    // Pseudo-constructor
    pub fn new(transfer_rate: i32, bufsize: i32, port: u16, early_terminate: u8) -> AbstractNetworkLayer {
        AbstractNetworkLayer { transfer_rate, bufsize, port, early_terminate }
    }

    ///
    /// Schedules Task to ThreadPool.
    ///
    pub fn schedule_task(&self, message: &str) -> () {
        println!("{}", &message);
    }
}
