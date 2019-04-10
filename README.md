<p align=center><img src=resources/dandelion_logo.svg alt=Dandelion width=200></p>


[![License: CC BY-NC-SA 
4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)

# 🚀 Dandelion : POSIX-compliant Microkernel Real-Time Operating System in Rust

**Dandelion**. Make your mistakes more reliable.

(or “Meet your constraints.”)

> Ainsi, toujours poussés vers de nouveaux rivages,
>
> Dans la nuit éternelle emportés sans retour,
>
> Ne pourrons-nous jamais sur l’océan des âges
>
> Jeter l’ancre un seul jour ?

## Table of contents

- :fallen_leaf: [Motivation](#motivation)
- 🌴 [Main design characteristics](#main design characteristics)
- 💐 [Technical choices](#technical choices)
- :ear_of_rice: [Getting started](#getting started)
  - :hibiscus: [Prerequisites](#prerequisites)
  - :cherry_blossom: [Installing]
- :sunflower: [Running the tests]
  - :tulip: [End to end tests]
  - :blossom: [Coding style tests]
- :herb: [Deployment]
- :maple_leaf: [Built tool](#build tool)
- 🌲 [Documentation](#documentation)
- 🌹 [File hierarchy]
- :seedling: [Contributing](#contributing)
- :cactus: [Versioning](#versioning)
- :leaves: [Authors](#authors)
- :four_leaf_clover: [License](#license)
- :deciduous_tree: [Acknowledgments](#acknowledgments)

## Motivation

This project aims to create a POSIX-compliant microkernel real-time operating system using the Rust language. It addresses a large area of design techniques and algorithms so as to reach three defined goals  :

- **deterministic** execution time of the programs (or subparts of them), allowing the processes to be managed accurately
- **correctness** of the time when a result is expected, to meet or miss the deadlines, where treatment marks the difference between soft and hard RTOS
- **predictability** of the deadline associated with a set of constraints to define an expected state of the system

## Main design characteristics

Dandelion is a hard time-sharing RTOS with a three-level scheduler
  - admitter (long-term) : estimate job completion time and accept it or not
  - swapper (medium-term) : swaps in and out the processes, depending on their activity
  - dispatcher (short-term) : decide which ready process to run and to which processor/core to assign

### EPSRC / AHRC principles of robotics

1. Robots should not be designed solely or primarily to kill or harm humans.
2. Humans, not robots, are responsible agents. Robots are tools designed to achieve human goals.
3. Robots should be designed in ways that assure their safety and security.
4. Robots are artifacts; they should not be designed to exploit vulnerable users by evoking an emotional response or dependency. It should always be possible to tell a robot from a human.
5. It should always be possible to find out who is legally responsible for a robot.

## Technical choices

### File formats

* configuration files : toml
* data exchange format : JSON
* documents format : Markdown

## Getting started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#Deployment) for notes on how to deploy the project on a live system.

### Prerequisites

First of all, you need to install the Rust toolchain, available on the [official website](https://www.rust-lang.org).
Then you need to install an emulator to run the RTOS. We recommand [Qemu](https://www.qemu.org/), but any other emulator able to read iso images is suitable for the job.

### Installing

A step by step series of examples that tell you how to get a development env running.
Say what the step will be

```
git clone https://github.com/AntoineSebert/dandelion
cd dandelion
```

If you want to clone dandelion in a particular folder, use

```
git clone https://github.com/AntoineSebert/dandelion location
cd location/dandelion
```

Then you need to build the project and generate an iso image of the RTOS.

```
cargo build
cargo run parameters
```

```
qemu ... dandelion.iso
```

End with an example of getting some data out of the system or using it for a little demo.

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

## Deployment

Add additional notes about how to deploy this on a live system.

## Build tool

* [cargo](http://www.example.com) - the official Rust build tool

## Documentation

The documentation for this repository can be found at https://github.com/AntoineSebert/dandelion/documentation

## File hierarchy

--{root}								- dandelion root directory.
  +--.gitignore							- elements ignored by *git*
  +--CODEOWNERS.txt					- responsible devs
  +--.CODE_OF_CONDUCT.md			- code of conduct for the project
  +--CONTRIBUTING.md				- contribution guidelines
  +--LICENSE.md						- license CC BY-NC-SA 4.0
  +--PULL_REQUEST_TEMPLATE.md		- pull request template
  +--README.md						- this file
  +--.github/							- Github specific files
  |  +--ISSUE_TEMPLATE/					- templates
  |    +--bug_report.md						- bug report template
  |    +--feature_request.md				- feature request
  +--documentation/						- documentation files
  |  +--html/							- HTML documentation files
  |  +--markdown/						- Markdown documentation files
  +--documents/							- miscellaneous documents
  |  +--references.bib						- bibliography
  |  +--palette.md						- project's visual identity
  |  +--diagrams/						- project UML diagrams
  |  +--palette.assets/
  |  +--submission_1/
  |  |  +--ethics_form.pdf
  |  |  +--project_proposal.pdf
  |  |  +--project_specification.md
  |  |  +--project_specification.pdf
  |  +--submission_2/
  |  |  +--literature_review.md
  |  |  +--literature_review.pdf
  |  |  +--literature_review_draft_core.md
  |  |  +--literature_review_draft_optional.md
  |  +--submission_3/
  |  |  +--requirements_analysis.md
  |  |  +--requirements_analysis.pdf
  |  +--submission_4/
  |  |  +--poster.heif
  |  |  +--poster.png
  |  |  +--poster.tiff
  |  |  +--presentation
  |  +--submission_5/
  |     +--final_report.pdf
  +--lectures/
  |  +--L1.md
  |  +--L2.md
  |  +--L3.md
  |  +--L4.md
  |  +--L5.md
  |  +--L6.md
  +--meetings/
  |  +--sample.md
  |  +--2018-09/
  |  |  +--27.md
  |  +--2018-10/
  |  |  +--04.md
  |  |  +--11.md
  |  |  +--18.md
  |  |  +--25.md
  |  +--2018-11/
  |  |  +--01.md
  |  |  +--08.md
  |  |  +--15.md
  |  |  +--22.md
  |  |  +--29.md
  |  +--2018-12/
  |  |  +--06.md
  |  |  +--13.md
  |  +--2019-01/
  |  +--2019-02/
  |  +--2019-03/
  |  +--2019-04/
  +--resources/
  |  +--dandelion_logo.png
  |  +--dandelion_logo.svg
  +--src/
  |  +--kernel/
  |  |  +--core/
  |  |  +-optional/
  |  |     +--filesystem/
  |  |     |  +--ipc_mechanism/
  |  |     |  |  +--optional/
  |  |     |  |  |  +--pipes/
  |  |     |  |  |  | +--named_pipes/
  |  |     |  |  |  +--queues/
  |  |     |  |  |  +--shared_memory/
  |  |     |  |  +--signals/
  |  |     |  |  +--sockets/
  |  |     |  +--scheduler/
  |  |     |  +--shell/
  |  |     |  |  +--commands/
  |  |     |  |  +--interpreter/
  |  |     |  +--virtual_memory_manager/
  |  |     +--io/
  |  |        +--keyboard/
  |  |        +--screen/
  |  |        +--speaker/
  |  +--modules/
  +--target/
  +--tests/

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/AntoineSebert/dandelion/tags). 

## Authors

* [**Antoine/Anthony Sébert**](https://github.com/AntoineSebert) - *Design*
	* I do not declare any conflict of interest. Besides trying to increase humanity's knowledge, this project is necessary for me to complete my cursus at The Robert Gordon University.
* [**Antoine/Anthony Sébert's clone**](https://github.com/AntoineSebert) - *Implementation*
* [**Antoine/Anthony Sébert's evil twin**](https://github.com/AntoineSebert) - *Bug creation*
* [**Antoine/Anthony Sébert's future self**](https://github.com/AntoineSebert) - *Testing*
* [**Antoine/Anthony Sébert from a parallel dimension**](https://github.com/AntoineSebert) - *Documentation*

## License

This project is licensed under the CC BY-NC-SA License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

* My supervisor, [Petrovski](https://orcid.org/0000-0002-0987-2791)
* M. Bartlett
* N. Wiratunga
* My friends
* My family