# linux-rust-core-utils

My implementation of the Linux core utilities written in Rust.

## Implemented Utilities

- [x] `echo` – Display a line of text
- [ ] `cat` – Concatenate and print files
- [ ] `ls` – List directory contents
- [ ] `cp` – Copy files and directories
- [ ] `rm` – Remove files or directories

## Building & Usage

Build the entire workspace or specific utilities from the root directory:

```bash
# Build all utilities in release mode
cargo build --release --workspace

# Build a single utility
cargo build --release -p echo

# Run a compiled binary
./target/release/echo "Hello, world!"
```

Move them into the `/usr/local/bin` directory:

```bash
sudo mv ./echo /usr/local/bin
```

## License

This project is under the kforkernel Use License (KUL) v1.0. See [LICENSE.md](LICENSE.md).
