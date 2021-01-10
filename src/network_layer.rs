// Rate,buf,port,early_term
pub const MAX_ARGS: i32 = 5;
pub const DEFAULTS: [i32; 4] = [100, 64, 8000, 0];

pub struct AbstractNetworkLayer {
    bufsize: i32,
    early_terminate: u8,
    port: u16,
    transfer_rate: i32,
    // 0-65535
}

impl AbstractNetworkLayer {
    // Pseudo-constructor
    pub fn new(bufsize: i32, early_terminate: u8, port: u16, transfer_rate: i32) -> AbstractNetworkLayer {
        AbstractNetworkLayer { bufsize, early_terminate, port, transfer_rate }
    }

    ///
    /// Schedules Task to ThreadPool.
    ///
    pub fn schedule_task(&self, message: &str) -> () {
        println!("{}", &message);
    }
}
