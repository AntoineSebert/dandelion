# Real-time Operating System in Rust

[Antoine Sébert](mailto:antoine.sb@orange.fr)[^author]

09/02/2019

> "*...*"

## Abstract

...

## Categories and Subject Descriptors

### ACM classifiers

...

### General Terms

...

### Keywords

...

## Introduction

...

## Design

...

## Methodology

### Qemu

To run the OS, the [QEMU](https://www.qemu.org/) emulator version 3.1.0 has been chosen.

### Rust

The implementation language choosen is [Rust](https://www.rust-lang.org/), for it has been guaranteed free from common problems related to memory (overflows, pointers) while remaining almost as fast as C/C++. Rust’s ecosystem is also quickly expanding, part because it supported by the Mozilla Foundation (especially in its product Firefox), and part because of the community that endlessly creates new open source software projects.
The toolchain (including the compiler and its most important components) can be installed via [rustup](https://rustup.rs/).
Note that the component **rust-src** is required to build this project, and can be installed with `rustup component add rust-src`.

#### Toolchain

The chosen toolchain is nightly-2019-02-03-x86_64-pc-windows-msvc. The cargo feature `publish-lockfile`, used in the dependancy [bootloader](https://crates.io/crates/bootloader) 0.3.13 requires a nightly version of Cargo.
|component|function|version|build info|
|-|-|-|-|
|[rustc](https://doc.rust-lang.org/rustc/index.html)|compiler| 1.34.0-nightly|8a57831a4 2019-02-02|
|[cargo](https://doc.rust-lang.org/cargo/index.html)|build tool|1.34.0-nightly|245818076 2019-01-27|
|[rustdoc](https://doc.rust-lang.org/rustdoc/index.html)|documentation generator|1.34.0-nightly|8a57831a4 2019-02-02|
|[rustfmt](https://github.com/rust-lang/rustfmt)|formatting tool|1.0.1-nightly|be135599 2018-12-10|
The OS image is build and run with [bootimage](https://crates.io/crates/bootimage) version 0.6.4, which operates atop of cargo.

## Results

unit tests
integration tests (src/bin)
bootimage test

## Discussion

...

## Conclusion

...

## Further research

...

## Acknowledgements

...

## References

[^author]: Student at School of Computing Science and Digital Media, [Robert Gordon University](https://www.rgu.ac.uk), Aberdeen, Scotland, United Kingdom

## Bibliography

...

## Annexes

...