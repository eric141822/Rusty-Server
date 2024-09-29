# Rusty Server

This is a simple multi-threaded server written in Rust. Shuts down gracefully after serving `x` amount of requests.

## Usage

Clone the repo and run `cargo run --release -- <max_requests>`. For example, `cargo run --release -- 2` will create a server that shuts down gracefully after serving 2 requests.
