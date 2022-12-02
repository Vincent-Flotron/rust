# Installation and first commands



## Tutorial
[installation](https://www.rust-lang.org/learn/get-started)



## Things to install
 
### Builder
#### On Windows
[Visual Studio C++ Build tools](https://visualstudio.microsoft.com/fr/visual-cpp-build-tools/)

#### On Ubuntu
```
sudo apt install build-essential
```

### Toolchain manager
[rustup](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)



## Updating rust

```bash
rustup update
```


## Building application

```bash
cargo build
```


## Running application

```bash
cargo run
```

## From VS code

Install the app "rust-analyzer". (v0.3.1238 was used this time)
Install the app "Rust Syntax". (v0.6.1 was used this time)

### Install a missing crate

```
cargo install chrono
``
