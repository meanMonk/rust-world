> Learn Rust By Following Book!

<h1 style="font-family:roboto;text-transform:uppercase;color:#ee00ff;">The Rust Programming Language</h1>


[follow](https://doc.rust-lang.org/book/ch00-00-introduction.html)


Rust also brings contemporary developer tools:
 - cargo: deps manager & build tool
 - rustfmt : formating tool for consisten coding styles across developers.
 - Rust Langauge Servers: IDE integration for code completion and inline error messages.

Applications / Uses: 
  Rust in production for a variety of tasks, 
   - including command line tools, 
   - web services, 
   - DevOps tooling, 
   - embedded devices, 
   - audio and video analysis and transcoding, 
   - cryptocurrencies, 
   - bioinformatics, 
   - search engines, 
   - Internet of Things applications, 
   - machine learning, 
   - even major parts of the Firefox web browser.


## Installation 

```bash
 curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## check version

```bash
 rustup --version
```
## Update

```bash
 rustup update
```

## local documentation

```bash
 rustup docs
```


# Cargo!
 - Cargo is rust's build system and package manager.

## To build the project
```bash
cargo build
```

## to run the project
```bash
target/debug/hello_cargo
```

## to buil and run at one time make use of 

```bash
cargo run
```