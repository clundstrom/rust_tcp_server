use std::net::TcpListener;

pub const MAX_ARGS: i32 = 5;
pub const DEFAULTS: [i32; 4] = [100, 64, 8000, 0];

pub struct AbstractNetworkLayer {
    pub bufsize: i32,
    pub early_terminate: u8,
    pub port: u16,
    pub transfer_rate: i32,
}

impl AbstractNetworkLayer {

    /// Schedules Task to ThreadPool.
    pub fn schedule_task(&self, message: &str) -> () {
        println!("{}", &message);
    }

    /// Opens a TCP listener
    pub fn accept_connections(&self) -> TcpListener {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = match TcpListener::bind(&addr) {
            Ok(res) => { res }
            Err(_) => { panic!("Could bind to {}", addr) }
        };
        log::info!("Bind on {addr}", addr = addr);

        listener
    }
}
