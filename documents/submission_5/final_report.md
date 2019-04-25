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

The chosen toolchain is the nightly version of the compiler for the host OS defined by the [target triplet](https://wiki.osdev.org/Target_Triplet) x86_64-pc-windows-msvc.

**Note on version numbers** : all version below “*1.0.0*“” and nightly versions are considered unstable and can break at any time for any reason (see [Semver](https://semver.org/) for more information about versioning).

| component                                                  | function                                               | version        |
| ---------------------------------------------------------- | ------------------------------------------------------ | -------------- |
| [rustc](https://doc.rust-lang.org/rustc/index.html)        | compiler                                               | 1.34.0-nightly |
| [cargo](https://doc.rust-lang.org/cargo/index.html)        | build tool                                             | 1.34.0-nightly |
| [rustdoc](https://doc.rust-lang.org/rustdoc/index.html)    | documentation generator                                | 1.34.0-nightly |
| [rustfmt](https://github.com/rust-lang/rustfmt)            | formatting tool                                        | 1.1.0-nightly  |
| [clippy](https://github.com/rust-lang/rust-clippy)         | linter                                                 | 0.0.212        |
| rust-src                                                   | Rust source code                                       | *null*         |
| llvm-tools-preview                                         | binary inspection and profiling, required by bootimage | 0.1.1          |
| [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) | cross-compiles `core`, builtins, and alloc             | 0.5.5          |
| [bootimage](https://github.com/rust-osdev/bootimage)       | creates a bootable disk image from a Rust OS kernel    | 0.7.3          |
The OS image is build and run with [bootimage](https://crates.io/crates/bootimage) version 0.6.4, which operates atop of cargo.

#### Dependencies

No top-level dependency (directly used to create this OS) can be considered stable, concerning either the build or the library API.

![graph](final_report.assets/graph.png)

#### The `no_std` context

| feature                                      | no_std | std  |
| -------------------------------------------- | ------ | ---- |
| heap (dynamic memory)                        | *      | ✓    |
| collections (Vec, HashMap, etc)              | **     | ✓    |
| stack overflow protection                    | ✘      | ✓    |
| runs init code before main                   | ✘      | ✓    |
| libstd available                             | ✘      | ✓    |
| libcore available                            | ✓      | ✓    |
| writing firmware, kernel, or bootloader code | ✓      | ✘    |

\* Only if you use the `alloc` crate and use a suitable allocator like [alloc-cortex-m](https://github.com/rust-embedded/alloc-cortex-m).

** Only if you use the `collections` crate and configure a global default allocator.

from https://rust-embedded.github.io/book/intro/no-std.html, retrieved 25/04/2019

## Results

unit tests
integration tests (src/bin)
bootimage test

## Discussion

...

## Conclusion

Any update on any component of the toolchain, from the Rust compiler itself to the lowest-level library of the dependency graph, can break the whole build process, making the source code impossible to compile (and hence run). … Everything is in constant evolution and stability is nothing but a sweet dream at the time I am writing those lines.

Yet, the highly active environment is very promising, both in terms of diversity of projects aimed towards any field of computing science, and in terms of dedication of maintainers … .

…

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