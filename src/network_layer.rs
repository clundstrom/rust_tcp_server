use std::net::{TcpStream, TcpListener, Shutdown};
use std::io::{Read, Write};

pub const MAX_ARGS: i32 = 5;
pub const DEFAULTS: [i32; 4] = [100, 64, 8000, 0];

// #[derive(Copy, Clone)]  <- either this or manual implementation
pub struct AbstractNetworkLayer {
    pub bufsize: i32,
    pub early_terminate: u8,
    pub port: u16,
    pub transfer_rate: i32,
}

/// Copy, clone for struct to avoid use after move error in closure
impl Copy for AbstractNetworkLayer {}

impl Clone for AbstractNetworkLayer {
    fn clone(&self) -> AbstractNetworkLayer {
        *self
    }
}

impl AbstractNetworkLayer {

    /// Echoes data
    pub fn echo(&self, mut stream: TcpStream) {
        let mut buf = vec![0; self.bufsize as usize];

        while match stream.read(&mut buf) {
            Ok(received) => {
                if received > 0 {
                    stream.write(&buf[0..received]).unwrap();
                    log::info!("{} bytes written", received);
                }
                true
            }
            Err(_) => {
                log::error!("Could not write to stream. Terminating connection {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
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
