# Microkernel Real-time Operating System in Rust

[Antoine Sébert](mailto:antoine.sb@orange.fr)[^author]

09/02/2019

> “The tools we use have a profound and devious influence on our thinking habits, and therefore on our thinking abilities”
> E. W. Dijkstra

## Abstract

This document describes a microkernel RTOS that aims to enforce the reliability of the system by taking advantage of the Rust programming language’s characteristics.

The system is able to schedule and run tasks with strict timed constraints, both periodic & aperiodic, and also supports non-realtime tasks without guaranty of completion. The size of the program has been kept relatively small to fit the majority of embedded devices.

The Rust language is very likely to become a tool of choice within the next couple of years, yet the temporary but pervasive instability of the development environment is a serious concern.

## Categories and Subject Descriptors

### ACM classifiers

• **Computer systems organization~Real-time operating systems**

• *Software and its engineering~Real-time schedulability*

• Software and its engineering~Multiparadigm languages

### Keywords

Real-Time, Operating System, RTOS, microkernel, Rust

## Introduction

Real-time Operating Systems are widely used in several sectors, including astronautics, mainframes, aeronautics, robotics, or embedded systems in general (and IoT in particular) [**…**], for their predictability and deterministic behavior [**…**]. They are typically more tolerant to failures and more predictable [**Aroca & Caurin, 2009**], and particular care is taken about the implementation and the tests.

However the emergence of modern programming languages for systems development, that intent to address the drawbacks of older programming languages [**Balasubramanian et al., 2017**], question the choice of the ubiquitous C/C++ or Java as default choices for such projects [**Orozco & Santos, 2012**]. This brings us to Rust: originally created by Graydon Hoare, who worked at the Mozilla Foundation, it is now developed by the Rust project developers, centered around the [language's Github repository](https://github.com/rust-lang/rust).

Designed to be fast and reliable, this language is memory safe, uses Resource Acquisition Is Initialization (RAII) to manage memory (avoiding the necessity of a garbage collector), supports polymorphism and natively supports concurrency [**Matsakis & Klock, 2014**]. It is often cited as a potential successor of C++.

Whereas the majority of current RTOS have been conceived when Rust did not exist, they are written in languages that show their limits when it comes to real-time computing, or lack the desirable features [**Burns & Wellings, 2001**]. Hence a RTOS implemented in Rust seems very promising. Very few attempts in this area have been made [**Heldring, 2018**], and yet real-time computing is a growing field in the need for a new generation of RTOS aimed to solve arising challenges.

It is planned to design and develop the kernel of a real-time operating system that focuses on the essential features and demonstrates the scheduling of multiple processes with time-bound constraints and static priority [**Buttazzo, 2011**]. The tasks can be aperiodic (deadline), periodic (interval) or non-realtime. Inter-process communication by message-passing [**…**] between the processes is also wished. The system will be written in pure Rust and take advantage of the language’s features to achieve performance and reliability [**…**].

## Methodology

### Toolchain and environment

#### Rust

Rust, for it has been guaranteed free from common problems related to memory (data races, null pointers, *etc*) [**Jung et al., 2017**] while remaining [almost as fast as C/C++]( https://benchmarksgame-team.pages.debian.net/benchmarksgame/faster/rust.html). Rust’s ecosystem is also quickly expanding, part because it supported by the Mozilla Foundation (especially in its product Firefox), and part because the community endlessly creates new open source [software projects](https://crates.io/). It has also a [very good reputation](https://insights.stackoverflow.com/survey/2019/#key-results) amongst the developers that use it.

**Note on version numbers** : all version below “*1.0.0*“ and nightly versions are considered unstable and can break at any time for any reason, according to the [Semver](https://semver.org/) versioning system [**Raemaekerset al., 2017**].

##### Rust toolchain

The toolchain (including the compiler and its most important components) can be installed via [rustup](https://rustup.rs/). The chosen toolchain is the nightly version of the compiler for the host OS defined by the [target triplet](https://wiki.osdev.org/Target_Triplet) *x86_64-pc-windows-msvc*.

| component                                                  | function                                               | version        |
| ---------------------------------------------------------- | ------------------------------------------------------ | -------------- |
| [rustc](https://doc.rust-lang.org/rustc/index.html)        | Rust compiler                                          | 1.34.0-nightly |
| [cargo](https://doc.rust-lang.org/cargo/index.html)        | package manager and build tool                         | 1.34.0-nightly |
| [rustfmt](https://github.com/rust-lang/rustfmt)            | formatting tool                                        | 1.1.0-nightly  |
| [clippy](https://github.com/rust-lang/rust-clippy)         | linter                                                 | 0.0.212        |
| rust-src                                                   | Rust source code                                       | *null*         |
| llvm-tools-preview                                         | binary inspection and profiling, required by bootimage | 0.1.1          |
| [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) | cross-compiles `core`, builtins, and alloc             | 0.5.5          |
| [bootimage](https://github.com/rust-osdev/bootimage)       | creates a bootable disk image from an OS binary        | 0.7.3          |

##### Dependencies

###### Graph

![graph](final_report.assets/graph.png)

###### Table

| Name                                                      | Description                                                  | version |
| --------------------------------------------------------- | ------------------------------------------------------------ | ------- |
| [bootloader](https://crates.io/crates/bootimage)          | Tool to create a bootable OS image from a kernel binary.<br />Enabling the feature `map_physical_memory` maps the physical memory into the virtual address space. All the physical memory can then easily be accessed by the kernel. This approach has been preferred to the 4-level page hierarchy (feature `recursive_page_table`) for it is simpler. | ^0.6    |
| [volatile](https://crates.io/crates/volatile)             | A simple volatile wrapper type.                              | ^0.2    |
| [uart_16550](https://crates.io/crates/uart_16550)         | Minimal support for uart_16550 serial output.                | ^0.2    |
| [x86_64](https://crates.io/crates/x86_64)                 | Support for x86_64 specific instructions (e.g. TLB flush), registers (e.g. control registers), and structures (e.g. page tables). | ^0.5    |
| [pic8259_simple](https://crates.io/crates/pic8259_simple) | Kernel-space interface to 8259 and 8259A Programmable Interrupt Controller. | ^0.1    |
| [integer-sqrt](https://crates.io/crates/integer-sqrt)     | This module contains the single trait `IntegerSquareRoot` and implements it for primitive integer types. | ^0.1    |
| [cmos](https://crates.io/crates/cmos)                     | A utility to read, write CMOS and RTC data. Standard library not required. The crate has been forked (https://github.com/AntoineSebert/cmos) to extend its features. | ^0.1    |
| [array-init](https://crates.io/crates/array-init)         | Safe wrapper for initializing fixed-size arrays.             | ^0.0    |
| [pc-keyboard](https://crates.io/crates/pc-keyboard)       | PS/2 keyboard interface library.                             | ^0.5    |
| [spin](https://crates.io/crates/spin)                     | Synchronization primitives based on spinning. They may contain data, are usable without `std`, and static initializers are available. | ^0.5    |
| [arraydeque](https://crates.io/crates/arraydeque)         | A ring buffer with a fixed capacity, which can be stored on the stack. | ^0.4    |
| [either](https://crates.io/crates/either)                 | The enum `Either` with variants `Left` and `Right` is a general purpose sum type with two cases. The crate’s default features have been disabled to work in a `no_std` context. | ^1.5    |
| [num-traits](https://crates.io/crates/num-traits)         | Numeric traits for generic mathematics in Rust. The crate’s default features have been disabled to work in a `no_std` context. | ^0.2    |
| [lazy_static](https://crates.io/crates/lazy_static)       | A macro for declaring lazily evaluated statics in Rust. Enabling the feature `spin_no_std` make the use of the crate possible in a `no_std` context. | ^1.3    |

##### The `no_std` context

Since standard libraries are developed for a particular implementation of a programming language, it depends on the operating system. But in the case of the creation of an operating system, no standard library is available, unless written by the developers. Within the context of Rust, that also includes `collections`, a module containing common data structures, and stack overflow protection.

There is also no dynamic memory (provided by the OS).

The following table retrieved from https://rust-embedded.github.io/book/intro/no-std.html sums up the main differences.

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

#### Target Specification

Since we are creating a custom OS, we have to define our own target platform in the file [x86_64-dandelion.json](..\..\x86_64-dandelion.json). No particular specification has been found for this filetype. The target architecture is then passed to Cargo as parameter by bootimage to use the cross-compilation feature of rustc.

```json
{
	"llvm-target": "x86_64-unknown-none",
	"data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
	"arch": "x86_64",
	"target-endian": "little",
	"target-pointer-width": "64",
	"target-c-int-width": "32",
	"os": "none",
	"executables": true,
	"linker-flavor": "ld.lld",
	"linker": "rust-lld",
	"panic-strategy": "abort",
	"disable-redzone": true,
	"features": "-mmx,-sse,+soft-float"
}
```

* `"llvm-target": "x86_64-unknown-none"` : the target is 64-bit architecture, unknown manufacturer, without OS

* `"data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128"` : the target … https://llvm.org/docs/LangRef.html#data-layout

* `"arch": "x86_64"` : the program runs on 64-bit architecture

* `"target-endian": "little"` : …

* `"target-pointer-width": "64"` : …

* `"target-c-int-width": "32"` : …

* `"os": "none"` : the program will run on bare metal

* `"executables": true` : …

Instead of using the platform's default linker (which might not support Linux targets), we use the cross platform [LLD](https://lld.llvm.org/) linker that is shipped with Rust for linking our kernel.

* `"linker-flavor": "ld.lld"` : …

* `"linker": "rust-lld"` : …

* `"panic-strategy": "abort"` : In case of kernel panic, the programs aborts because stack unwinding (popping one or more frames off the stack to resume execution elsewhere in the program) is not supported.

* `"disable-redzone": true` : the redzone is an optimization that makes place for temporary data on a function's stack. In case of an interruption, the exception handler can overwrite the data in the redzone, leading to stack corruption. Thus it has been disabled.

* `"features": "-mmx,-sse,+soft-float"` : the MMX and SSE instructions set are disabled because they lead to significant performance loss. …

disable simd instructions

#### Qemu

To run the OS, the [QEMU](https://www.qemu.org/) emulator version 3.1.0 has been chosen. It is widely-used and aims for speed, plus supports a [large choice of hardware platforms](https://wiki.qemu.org/Documentation/Platforms).

### Design

#### Essential characteristics

This project aims to reach three defined goals  :

- **deterministic** execution time of the programs (or subparts of them), allowing the processes to be managed accurately

- **correctness** of the time when a result is expected, to meet or miss the deadlines, where treatment marks the difference between soft and hard RTOS

- **predictability** of the deadline associated with a set of constraints to define an expected state of the system

##### Microkernel OS architecture

...

##### Hard realtime

...

##### Time-sharing

...

##### Scheduling algorithms

...

#### Operating System hierarchy

Rust/Cargo projects are modular by default, with the project hierarchy following the file hierarchy within the guest OS.

The entry point is defined in *main.rs*, while *lib.rs* stores code to be used as part of the crate’s library and *mod.rs* refers to the local module’s root. Every other file is a module by itself, and submodules within files can also be defined. As a result the code organization sticks to the final organization of the program.

```
--src/
  +--bin/
  |  +--test-basic-boot.rs
  |  +--test-exception-breakpoint.rs
  |  +--test-exception-double-fault-stack-overflow.rs
  |  +--test-firm-deadline-exception.rs
  |  +--test-hard-deadline-exception.rs
  |  +--test-panic.rs
  |  +--test-soft-deadline-exception.rs
  |  +--test-task-remaining-exception.rs
  |  +--test-time-remaining-exception.rs
  +--kernel/
  |  +--ipc/
  |  |  +--mod.rs
  |  +--scheduler/
  |  |  +--mod.rs
  |  |  +--admitter.rs
  |  |  +--dispatcher.rs
  |  |  +--swapper.rs
  |  +--shell/
  |  |  +--mod.rs
  |  +--vmm/
  |  |  +--mod.rs
  |  |  +--gdt.rs
  |  |  +--memory.rs
  |  +--mod.rs
  |  +--acpi.rs
  |  +--interrupt.rs
  |  +--process.rs
  |  +--serial.rs
  |  +--time.rs
  |  +--vga_buffer.rs
  +--lib.rs
  +--main.rs
```

#### Booting steps

```flow
st=>start: Start
op0=>operation: Kernel build
op1=>operation: Bootloader build
op2=>operation: Link kernel to bootloader
op3=>operation: Create disk image
op4=>operation: Run disk image in QEMU
e=>end

st->op0->op1->op2->op3->op4->e
```

#### Activity steps

```flow
st=>start: Start
io0=>inputoutput: Hello World
sub0=>subroutine: Initialize `gdt` and interrupts
sub1=>subroutine: Map kernel inot memory
cond=>condition: Processes to run ?
sub2=>subroutine: Schedule and run processes
io1=>inputoutput: It did not crash
sub3=>subroutine: infinite loop
e=>end

st->io0->sub0->sub1->cond
cond(yes)->sub2->io1->sub3->e
cond(no)->e
```

## Results

### Features

#### RTC and time-duration

So as to work with timestamps as accurate as possible, the real-time clock has been preferred to the timer. However a dummy timer’s interrupt handler is present in the OS, making the component ready-to-use. Both timer and real-time clock are managed through the PIC controller.

The `cmos` crate provide an interface to use the real-time clock. In adequacy to the registers in the component, a timestamp is represented as follows :

```rust
pub struct RTCDateTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: usize,
}
```

Several functions have been implemented perform conversions between `RTCDateTime` and `Duration`, part of `core`. Optimizations are … .

#### Tasks with time-bound constraints

##### Process states

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
	Limbo(Limbo),
	MainMemory(MainMemory),
	SwapSpace(SwapSpace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limbo {
	Creating,
	Killed,
	Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMemory {
	Ready,
	Running,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapSpace {
	Interrupted,
	Suspended,
	Delayed,
}
```

##### Process priority

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum PRIORITY {
	HIGH,
	MEDIUM,
	LOW,
}
```
##### Process as type aliases

```rust
pub type Arguments<'a> = &'a [&'a str];

pub type Periodic = (Duration, Duration, RTCDateTime);
pub type Aperiodic = (Duration, RTCDateTime, Option<RTCDateTime>);

pub type Info = (State, Duration, RTCDateTime);
pub type Constraint = (Option<Either<Periodic, Aperiodic>>, PRIORITY);

pub type Metadata = (Constraint, Info);

pub type Runnable = fn(Arguments) -> u64;
pub type Task = (Metadata, Runnable);

pub type Job<'a> = (Metadata, &'a [&'a Runnable]);
pub type Group<'a> = &'a [&'a Task];
```

#### Scheduler

...

```sequence
Alice->Bob: Hello Bob, how are you?
Note right of Bob: Bob thinks
Bob-->Alice: I am good thanks!
```

### Commands

Assuming the current working directory is the operating system’s directory.

Build & run :
````
bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04
````

Create a bootable device from the kernel image (with *sdX* as target device mounting point):
````
dd if=target/x86_64-dandelion/debug/bootimage-dandelion.bin of=/dev/sdX && sync
````

### Tests

#### Command

````
bootimage test
````

### Code statistics

#### Command

```
tokei ./src --files
```

#### Table

| Language                                            | Files | Lines | Code | Comments | Blanks |
| --------------------------------------------------- | ----- | ----- | ---- | -------- | ------ |
| Rust                                                | 27    | 2214  | 1531 | 338      | 345    |
| `lib.rs`                                            |       | 29    | 16   | 7        | 6      |
| `main.rs`                                           |       | 112   | 66   | 25       | 21     |
| `bin\test-basic-boot.rs`                            |       | 35    | 24   | 5        | 6      |
| `bin\test-exception-breakpoint.rs`                  |       | 39    | 28   | 5        | 6      |
| `bin\test-exception-double-fault-stack-overflow.rs` |       | 70    | 53   | 5        | 12     |
| `bin\test-firm-deadline-exception.rs`               |       | 41    | 31   | 5        | 5      |
| `bin\test-hard-deadline-exception.rs`               |       | 41    | 31   | 5        | 5      |
| `bin\test-panic.rs`                                 |       | 29    | 19   | 5        | 5      |
| `bin\test-soft-deadline-exception.rs`               |       | 41    | 31   | 5        | 5      |
| `bin\test-task-remaining-exception.rs`              |       | 41    | 31   | 5        | 5      |
| `bin\test-time-remaining-exception.rs`              |       | 41    | 31   | 5        | 5      |
| `kernel\acpi.rs`                                    |       | 6     | 1    | 4        | 1      |
| `kernel\interrupts.rs`                              |       | 291   | 180  | 70       | 41     |
| `kernel\mod.rs`                                     |       | 20    | 10   | 7        | 3      |
| `kernel\process.rs`                                 |       | 155   | 109  | 12       | 34     |
| `kernel\serial.rs`                                  |       | 46    | 30   | 9        | 7      |
| `kernel\time.rs`                                    |       | 101   | 70   | 12       | 19     |
| `kernel\vga_buffer.rs`                              |       | 243   | 182  | 25       | 36     |
| `kernel\scheduler\admitter.rs`                      |       | 121   | 87   | 12       | 22     |
| `kernel\scheduler\dispatcher.rs`                    |       | 275   | 227  | 22       | 26     |
| `kernel\scheduler\mod.rs`                           |       | 186   | 136  | 20       | 30     |
| `kernel\scheduler\swapper.rs`                       |       | 86    | 38   | 29       | 19     |
| `kernel\ipc\mod.rs`                                 |       | 4     | 0    | 4        | 0      |
| `kernel\shell\mod.rs`                               |       | 4     | 0    | 4        | 0      |
| `kernel\vmm\gdt.rs`                                 |       | 54    | 40   | 4        | 10     |
| `kernel\vmm\memory.rs`                              |       | 96    | 58   | 23       | 15     |
| `kernel\vmm\mod.rs`                                 |       | 7     | 2    | 4        | 1      |
| Total                                               | 27    | 2214  | 1531 | 338      | 345    |

## Discussion

Talk about dandelion’s limitations …

The main drawback encountered in this project is the relative immaturity of the ecosystem. Any update on any component of the toolchain, from the Rust compiler itself to the lowest-level library of the dependency graph, can break the whole build process, making the source code impossible to compile (and hence run). Everything is in constant evolution [see dep graph/table] and a particular care must be taken when choosing components to use, leading to *ad hoc* choices in terms of libraries and implementations of features. Anytime a feature has to be added, a search on the ecosystem must be performed so as to eventually select a library that both suits the project and works correctly.

Yet, the [highly active environment](https://github.blog/2018-11-15-state-of-the-octoverse-top-programming-languages/) is very promising, both in terms of diversity of projects aimed towards [any field of computing science](https://crates.io/category_slugs), and in terms of [activity of contributors](https://madnight.github.io/githut/), especially taking into account that the first stable version of the language was [released in 2015](https://blog.rust-lang.org/2015/05/15/Rust-1.0.html) (it is also the most loved programming language since 2016 on the platform Github [https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted, https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted, https://insights.stackoverflow.com/survey/2018#most-loved-dreaded-and-wanted, https://insights.stackoverflow.com/survey/2019#most-loved-dreaded-and-wanted]).

Another consideration is the language’s learning curve, which can constitute a fence that even developers moving from C++ might find hard to cross (it is not uncommon to describe the experience with the language as “painful” [general feeling from reading articles/comments on the internet, cannot be formally proven]). One does not simply walk into Rust’s ownership system, and even the object-oriented aspect can be confusing at first (composition over inheritance) [**Erich et al., 1994**], let alone the [functional-programming flavor](https://www.fpcomplete.com/blog/2018/10/is-rust-functional) of many code segments [**Poss 2014**].

Because of the nature of the project, the program runs on bare metal and has to interact directly with the hardware, thus heavily relies on [`unsafe` blocks](https://doc.rust-lang.org/nomicon/). These blocks may not pose a threat to the system’s stability for hardware is often seen as more reliable [evidece ?], but they cannot be considered as safe in the context of Rust. It results a lack of of compile-time checkings (allowing to write unsafe code) in critical sections that have to be carefully written.

Also, the online resource on operating systems (and real-time computing) is C-oriented (but yet very useful).   Already being familiar with operating systems theory and implementation is highly recommended, for understanding the code snippets considerably speed up development. Note that several projects aim to create efficient transpilers from C to Rust that preserve semantics and leave to the developer the task to make the code safe (https://github.com/jameysharp/corrode, https://github.com/immunant/c2rust).

… talk about achievements

## Conclusion

Originally suited for application development in a desktop environment (like Mozilla’s browser engine Servo for Firefox [**Anderson et al., 2015**]), Rust is growing towards higher application layers (web-oriented) and lower application layers (embedded/microcontrollers) [https://blog.rust-lang.org/2018/05/15/Rust-turns-three.html], [**Uzlu & Şaykol, 2017**]. Specifically embedded systems are officially subject to dedicated efforts, both from the language’s development team and the community (https://www.rust-lang.org/what/embedded, https://rust-embedded.org/).

The Rust community seems quite active, with many contributors and online places (forums, IRC channels, [subreddit](https://www.reddit.com/r/rust/)) where people exchange ideas. A [YouTube channel](https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA) also includes several conferences on specific topics. The most important operating system project written in Rust, [Redox](https://www.redox-os.org/), is an Unix-like OS. There are also a few more projects, some of them for educational purposes, plus several kernels or microkernels. The temporary immaturity issues are very likely to be solved within the next couple of years (https://blog.rust-lang.org/2019/04/23/roadmap.html). However a similar project is believed to be feasible by a team of skilled programmers and engineers with a determined goal.

In the actual context of growth of the Internet of Things and the rise of smart devices within that field (https://iot-analytics.com/state-of-the-iot-update-q1-q2-2018-number-of-iot-devices-now-7b/, https://www.cisco.com/c/en/us/solutions/collateral/service-provider/global-cloud-index-gci/white-paper-c11-738085.html, https://www.gartner.com/en/newsroom/press-releases/2017-02-07-gartner-says-8-billion-connected-things-will-be-in-use-in-2017-up-31-percent-from-2016, https://www.statista.com/statistics/471264/iot-number-of-connected-devices-worldwide/), the need for a high-productivity language is accentuated. Let alone the aeronautics and space industries (from big companies to simple contractors) that have similar demands [**Dvorak et al., 2004**].

## Acknowledgements

...

## References

[^author]: Student at School of Computing Science and Digital Media, [Robert Gordon University](https://www.rgu.ac.uk), Aberdeen, Scotland, United Kingdom. https://orcid.org/0000-0003-3096-6036

## Annexes

### Sources

 [lib.rs](..\..\src\lib.rs) 

 [main.rs](..\..\src\main.rs) 

#### Kernel

 [acpi.rs](..\..\src\kernel\acpi.rs) 

 [interrupts.rs](..\..\src\kernel\interrupts.rs) 

 [mod.rs](..\..\src\kernel\mod.rs) 

 [process.rs](..\..\src\kernel\process.rs) 

 [serial.rs](..\..\src\kernel\serial.rs) 

 [time.rs](..\..\src\kernel\time.rs) 

 [vga_buffer.rs](..\..\src\kernel\vga_buffer.rs) 

##### ipc

 [mod.rs](..\..\src\kernel\ipc\mod.rs) 

##### scheduler

 [admitter.rs](..\..\src\kernel\scheduler\admitter.rs) 

 [dispatcher.rs](..\..\src\kernel\scheduler\dispatcher.rs) 

 [mod.rs](..\..\src\kernel\scheduler\mod.rs) 

 [swapper.rs](..\..\src\kernel\scheduler\swapper.rs) 

##### shell

 [mod.rs](..\..\src\kernel\shell\mod.rs) 

##### vmm

 [gdt.rs](..\..\src\kernel\vmm\gdt.rs) 

 [memory.rs](..\..\src\kernel\vmm\memory.rs) 

 [mod.rs](..\..\src\kernel\vmm\mod.rs) 

### Tests

 [test-basic-boot.rs](..\..\src\bin\test-basic-boot.rs) 

 [test-exception-breakpoint.rs](..\..\src\bin\test-exception-breakpoint.rs) 

 [test-exception-double-fault-stack-overflow.rs](..\..\src\bin\test-exception-double-fault-stack-overflow.rs) 

 [test-firm-deadline-exception.rs](..\..\src\bin\test-firm-deadline-exception.rs) 

 [test-hard-deadline-exception.rs](..\..\src\bin\test-hard-deadline-exception.rs) 

 [test-panic.rs](..\..\src\bin\test-panic.rs) 

 [test-soft-deadline-exception.rs](..\..\src\bin\test-soft-deadline-exception.rs) 

 [test-task-remaining-exception.rs](..\..\src\bin\test-task-remaining-exception.rs) 

 [test-time-remaining-exception.rs](..\..\src\bin\test-time-remaining-exception.rs) 