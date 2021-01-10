## Simple TCP echo server in Rust

### Usage

* bufsize in bytes.
* early_terminate: 1/0
* transfer rate: max writes/s
* Port 0-65535

Rust webserver
```
cargo build
cargo run bufsize=50 early_terminate=1 transfer_rate=3  port=8000
```

Python client

* Specify host and matching port.

```
python src/socket_test.py
```

Memory footprint ~0.6mb




