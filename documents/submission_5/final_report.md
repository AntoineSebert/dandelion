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

QEMU emulator version 3.1.0 (v3.1.0-11736-g7a30e7adb0-dirty)

### Rust

The implementation language choosen is [Rust](https://www.rust-lang.org/), for ... (copypaste stuff written before).
The toolchains (including the compiler and its most important components) can be installed via [rustup](https://rustup.rs/).
Note that the component **rust-src** is required to build this project, and can be installed with `rustup component add rust-src`.

#### Toolchain

The chosen toolchain is nightly-2019-02-03-x86_64-pc-windows-msvc. The cargo feature `publish-lockfile`, used in the dependancy [bootloader](https://crates.io/crates/bootloader) 0.3.13 requires a nightly version of Cargo.
|component|function|version|build info|
|-|-|-|-|
|[rustc](https://doc.rust-lang.org/rustc/index.html)|compiler| 1.34.0-nightly|8a57831a4 2019-02-02|
|[cargo](https://doc.rust-lang.org/cargo/index.html)|build tool|1.34.0-nightly|245818076 2019-01-27|
|[rustdoc](https://doc.rust-lang.org/rustdoc/index.html)|documentation generator|1.34.0-nightly|8a57831a4 2019-02-02|
|[rustfmt](https://github.com/rust-lang/rustfmt)|formatting tool|1.0.1-nightly|be135599 2018-12-10|
The OS image is build and run with [bootimage](https://crates.io/crates/bootimage) version 0.6.4.

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