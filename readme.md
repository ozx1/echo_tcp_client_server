This is a simple echo TCP server and client written in rust

# Usage

### clone the repo by using
```bash
git clone https://github.com/ozx1/echo_tcp_client_server.git

cd echo_tcp_client_server
```

### then run the server by 
```bash
cargo run --bin server [addr]
```
it will run on 127.0.0.1:8080 by default

### run the client in another terminal
```bash
cargo run --bin client [addr]
```
it will connect to 127.0.0.1:8080 by default



