<p align=center><img src=resources/dandelion_logo.svg alt=Dandelion width=200></p>

[![License: CC BY-NC-SA
4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)

# 🚀 Dandelion : Microkernel Real-Time Operating System in Rust

**Dandelion**. Meet your constraints.

> Ainsi, toujours poussés vers de nouveaux rivages,
>
> Dans la nuit éternelle emportés sans retour,
>
> Ne pourrons-nous jamais sur l’océan des âges
>
> Jeter l’ancre un seul jour ?

## Table of contents

- [🚀 Dandelion : Microkernel Real-Time Operating System in Rust](#-dandelion--microkernel-real-time-operating-system-in-rust)
  - [Table of contents](#table-of-contents)
  - [Motivation](#motivation)
  - [Main design characteristics](#main-design-characteristics)
  - [Technical choices](#technical-choices)
  - [Getting started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installing](#installing)
    - [Running the operating system](#running-the-operating-system)
  - [Running the tests](#running-the-tests)
  - [License](#license)

## Motivation

This project aims to create a microkernel real-time operating system using the Rust language. It addresses a large area of design techniques and algorithms so as to reach three defined goals  :

- **deterministic** execution time of the programs (or subparts of them), allowing the processes to be managed accurately
- **correctness** of the time when a result is expected, to meet or miss the deadlines, where treatment marks the difference between soft and hard RTOS
- **predictability** of the deadline associated with a set of constraints to define an expected state of the system

## Main design characteristics

Dandelion is a hard time-sharing RTOS with a three-level scheduler
  - admitter (long-term) : estimate job completion time and accept it or not
  - swapper (medium-term) : swaps in and out the processes, depending on their activity
  - dispatcher (short-term) : decide which ready process to run and to which processor/core to assign

## Technical choices

## Getting started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#Deployment) for notes on how to deploy the project on a live system.

### Prerequisites

First of all, you need to install the Rust toolchain, available on the [official website](https://www.rust-lang.org).
Then you need to install an emulator to run the RTOS. We recommand [Qemu](https://www.qemu.org/), but any other emulator able to read iso images is suitable for the job.

### Installing

First you need to clone the repository:

```
git clone https://github.com/AntoineSebert/dandelion
cd dandelion
```

Then you need to build the project and generate an iso image of the RTOS:

```
cargo update
cargo build
```

### Running the operating system

Once all previous steps have been completed successfully, simply run the project:

```
cargo run
```

## Running the tests

```
cargo test
```

## License

This project is licensed under the CC BY-NC-SA License - see the [LICENSE.md](LICENSE.md) file for details.
