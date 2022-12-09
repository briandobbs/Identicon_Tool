# Identicon_Tool
An identicon tool written in Rust.

A small command line utility that takes a string and creates an identicon based on that string. The output should always be identical, given the same input. The default identicon algorithm used in this tool creates a grid of 25 x 25 squares, with each square 10 x 10 pixels, and the total grid is 250 x 250 pixels. The grids are colored according to the identicon algorithm, and each identicon is symmetrical along its vertical center and horizontal center axis.

Tool run the identicon tool, use `Cargo run -- {text}`. If your text has spaces in it, make sure to surround it with quotes, like below:

`Cargo run -- "hello world"`

`cargo build --target wasm32-wasi`

`wasmtime --dir=. target/wasm32-wasi/debug/identicon_tool.wasm "some more text" "colorful"`
