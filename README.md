<a href="https://github.com/AntoineSebert/RTOS-dandelion"><img src="resources/dandelion-logo.png" alt="Dandelion" width="200"></a>

# 🚀 Dandelion - POSIX-compliant Microkernel Real-Time Operating System in Rust

**Dandelion**. Make your mistakes more reliable.

[![License: CC BY-NC-SA 
4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)

------

## Table of contents
- :fallen_leaf: [Motivation](#motivation)
- 🌴 [Main design characteristics]
- 💐 [Technical choices]
- :ear_of_rice: [Getting started]
  - :hibiscus: [Prerequisites]
  - :cherry_blossom: [Installing]
- :sunflower: [Running the tests]
  - :tulip: [End to end tests]
  - :blossom: [Coding style tests]
- :herb: [Deployment]
- :maple_leaf: [Built tool]
- 🌲 [Documentation]
- :seedling: [Contributing](#contributing)
- :cactus: [Versioning](#versioning)
- :leaves: [Authors](#authors)
- :four_leaf_clover: [License](#license)
- :deciduous_tree: [Acknowledgments](#acknowledgments)

------

## Motivation
This project aims to create a POSIX-compliant microkernel real-time operating system using the Rust language. It addresses a large area of design techniques and algorithms so as to reach three defined goals  :

- **deterministic** execution time of the programs (or subparts of them), allowing the processes to be managed accurately
- **correctness** of the time when a result is expected, to meet or miss the deadlines, where treatment marks the difference between soft and hard RTOS
- **predictability** of the deadline associated with a set of constraints to define an expected state of the system

------

## Main design characteristics

...

time-driven
hard RTOS

------

## Technical choices

* configuration files : toml
* data exchange format : JSON
* documents format : Markdown
* 
[btrfs](https://github.com/kdave/btrfs-devel/tree/master/fs/btrfs)

------

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.
```
git clone https://github.com/AntoineSebert/RTOS-dandelion
make ... ?
run dandelion
```

### Prerequisites
What things you need to install the software and how to install them. Qemu ?

```
Give examples
```

### Installing
A step by step series of examples that tell you how to get a development env running.

Say what the step will be

```
Give the example
```

And repeat

```
until finished
```

End with an example of getting some data out of the system or using it for a little demo.

------

## Running the tests
Explain how to run the automated tests for this system.

### End to end tests
Explain what these tests test and why.

```
Give an example
```

### Coding style tests
Explain what these tests test and why.

```
Give an example
```

------

## Deployment

Add additional notes about how to deploy this on a live system.

------

## Built tool

* [...](http://www.example.com) - …

------

## Documentation

...

------

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

------

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

------

## Authors
* [**Antoine/Anthony Sébert**](https://github.com/AntoineSebert) - *Design*
* [**Antoine/Anthony Sébert's clone**](https://github.com/AntoineSebert) - *Implementation*
* [**Antoine/Anthony Sébert's evil twin**](https://github.com/AntoineSebert) - *Bug creation*
* [**Antoine/Anthony Sébert's future self**](https://github.com/AntoineSebert) - *Testing*
* [**Antoine/Anthony Sébert from a parallel dimension**](https://github.com/AntoineSebert) - *Documentation*

------

## License
This project is licensed under the CC BY-NC-SA License - see the [LICENSE.md](LICENSE.md) file for details.

-------

## Acknowledgments
* My supervisor, [Andrei Petrovski](https://orcid.org/0000-0002-0987-2791)
* My friends for being here
* My family