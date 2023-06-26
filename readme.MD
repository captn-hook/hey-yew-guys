# To get started with yew 
# https://yew.rs/docs/getting-started/introduction
Install WebAssembly target
Rust can compile source codes for different "targets" (e.g. different processors). The compilation target for browser-based WebAssembly is called wasm32-unknown-unknown. The following command will add the WebAssembly target to your development environment.

rustup target add wasm32-unknown-unknown

Install Trunk
Trunk is the recommended tool for managing deployment and packaging, and is used throughout the documentation and examples.

# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk

# Run locally with
## trunk serve

# Build for production with
# todo()!