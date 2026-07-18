FROM mcr.microsoft.com/devcontainers/rust:latest

# [Optional] Uncomment this section to install additional packages.
#RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#    && apt-get -y install --no-install-recommends python-is-python3

# Install the Rust REPL - https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md
RUN cargo install --locked evcxr_repl

# Install the Rust WASM target - https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32-unknown-unknown.html
RUN rustup target add wasm32-unknown-unknown

# Add ./bin to PATH
ENV PATH="./bin:${PATH}"
