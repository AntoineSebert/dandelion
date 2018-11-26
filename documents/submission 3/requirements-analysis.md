# Requirements analysis

by order of priority

## Must

### Virtual memory manager

- the RTOS will ensure process isolation, memory safety, and the load/unload of process' data/context
- segmented paging with P-LRU or LRU-k for page replacement
- reach a very low page miss ratio (advance a precise value would be too hazardous for it is dependant on the environment characteristics)
- one month estimation

### Scheduler

- the scheduler will have a twofold aim: decide whether or not accept new jobs, and assign processor cores to jobs
- will use EDF for short-term scheduling and RM for jobs acceptance
- cause all jobs to meet their deadlines
- two weeks estimation

### IPC core

- bring to jobs a framework to communicate with each other
- using signals, POSIX sockets
- provide "confidentiality, integrity, and availability of information"1
- allow processes to exchange data free from "unauthorized access, use, disclosure, disruption, modification, or destruction"2
- two weeks estimation

## Should

### shell

- create a command interpreter based on UNIX shells
- the shell will include a recursive descent LALR parser with bottom-up passes
- it will be generated with yacc + lex or gnu bison + flex
- interact with the RTOS using a command interpreter
- one week estimate

### shell commands

- create a set of basic commands to interact with the RTOS, based on Linux commands
- the names and the syntax will be similar
- commands strictly behave as expected
- one month estimate

## Could

### IPC optional

- bring to jobs a framework to communicate with each other
- using message queues, pipes, named pipes, shared memory
- provide "confidentiality, integrity, and availability of information"1
- allow processes to exchange data free from "unauthorized access, use, disclosure, disruption, modification, or destruction"2
- three weeks estimation

### filesystem

- provide an abstraction layer to manage files
- will use btrfs or an ad hoc solution
- store and retrieve data on a storage device
- three weeks estimate

### I/O

- create an I/O interface to manage I/O devices
- communication with device drivers
- implement an interface with a screen, keyboard, and speaker
- processes can drive I/O devices safely and concurrently
- one month estimate

## Won't

### network protocol

- a network protocol to create a communication system where entities can exchange information
- out of the scope of his project

### GUI

- use the GPU to render objects on the screen, and create a window manager
- not necessary

[1]: ISO/IEC 27000:2009
[2]: CNSS, 2010