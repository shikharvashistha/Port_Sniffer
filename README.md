# Port_Sniffer
Port_Sniffer CLI rustlang

## Usage
We'll use cargo run -- -h (we use -- to pass arguments to the executable and not cargo command)

cargo run -- -h //help screen
cargo run -- -j 100 192.168.1.1 //how many threads to use
cargo run -- 192.168.1.1 //calling code on ip with default threads = 4
