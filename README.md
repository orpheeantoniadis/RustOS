# RustOS
RustOS is a kernel written in Rust

## User manual

### Linux configuration
    sudo apt-get install curl nasm qemu
    
### Rust configuration
    curl https://sh.rustup.rs -sSf | sh
    echo source $HOME/.cargo/env >> ~/.bashrc
    rustup override add nightly
    cargo install xargo
    rustup component add rust-src

### Build
    make build
    
### Usage
    make run
    
## Resources
* [Rust book first edition](https://doc.rust-lang.org/book/first-edition)
* [Rust book second edition](https://doc.rust-lang.org/book/second-edition)
* [Cargo book](https://doc.rust-lang.org/cargo)
* [Target option](https://doc.rust-lang.org/1.1.0/rustc_back/target/struct.Target.html)
* [Target i386 example](https://github.com/rust-lang/rust/issues/33497)
* [__floatundisf issue](https://users.rust-lang.org/t/kernel-modules-made-from-rust/9191/2)
* [Writing an OS in Rust](https://os.phil-opp.com)
* [Writing an OS in Rust (Second Edition)](https://os.phil-opp.com/second-edition)