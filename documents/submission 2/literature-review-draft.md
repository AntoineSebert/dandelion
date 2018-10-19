# Search for relevant papers - DONE

* Real-Time Operating Systems
  * Scheduling
  * VxWorks
* Virtual memory management
* Microkernel Operating Systems
* IPC
* File system

# Make notes, identify techniques, pros and cons - WIP

## Core features

### IPC

#### (1989) Preserving and Using Context Information in Interprocess Communication

> When processes in a network communicate, the messages they exchange define a partial ordering of externally visible events. While the significance of this partial order in distributed computing is well understood, it has not been made an explicit part of the communication substrate upon which distributed programs are implemented. This paper describes a new interprocess communication mechanism, called Psync, that explicitly encodes this partial ordering with each message. The paper shows how Psync can be efficiently implemented on an unreliable communications network, and it demonstrates how conversations serve as an elegant foundation for ordering messages exchanged in a distributed computation and for recovering from processor failures.

- general notes
  * distributed-systems oriented
- techniques
  * *null*
- pros
  * preserve partial ordering of messages exchanged among a collection of processes
  * fault-tolerant (in a distributed system, probably in a modern computer as well -> multi-core processor)
  * pseudosynchronous
  * lightweight
  * low-level
  * quite performant
- cons
  * focus about network characteristics/issues
  * maybe for one-to-one communication
  * old
- supervisor notes
  * 

#### (1990) On Global-Time and Inter-process Communication

> How should time be represented in models for inter-process communication? The global-time axiom implies that all events can be represented by intervals on one time-axis. Its use simplifies the analysis of protocols and allows for intuitive proofs of their properties. On the other hand,some researchers believe it is too strong an assumption which should be avoided. In order to suggest an answer to this question we study the notion of a system-execution introduced by Lamport. We develop a practical tool which enables the investigation of protocols and allows for intuitive proofs of their properties. We apply our approach to prove that in many cases the global time axiom can be safely used. The main mathematical tool we employ is the theory of interval partial orders and we prove some new results along a line that goes back to the work of Russell and Wiener.

- general notes
  * theoretical
  * **probably irrelevant**
- techniques
  * *null*
- pros
  * useful in distributed systems
- cons
  * focused on distributed systems/protocols
  * old
- supervisor notes
  * 

#### (1995) The Real-Time PublisherSubscriber Inter-Process Communication

> Distributed real-time systems are becoming more pervasive in many domains including process control, discrete manufacturing, defense systems, air traffic control, and online monitoring systems in medicine. The construction of such systems, however, is impeded by the lack of simple yet powerful programming models and the lack of efficient, scalable, dependable and analyzable interfaces and their implementations. We argue that these issues need to be resolved with powerful application-level toolkits similar to that provided by ISIS. We consider the inter-process communication requirements which form a fundamental block in the construction of distributed real-time systems. We propose the real-time publisher/subscriber model, a variation of group-based programming and anonymous communication techniques, as a model for distributed real-time inter-process communication which can address issues of programming ease, portability, scalability and analyzability. The model has been used successfully in building a software architecture for building upgradable real-time systems. We provide the programming interface, a detailed design and implementation details of this model along with some preliminary performance benchmarks. The results are encouraging in that the goals we seek look achievable.

- general notes
  * **probably irrelevant**
- techniques
  * POSIX
  * UDP
- pros
  * fault-tolerant
- cons
  * focuses on Distributed Real-Time Systems
- supervisor notes
  * 

#### (2001) DIM, a portable, light weight package for information publishing, data transfer and inter-process communication

> The real-time systems of HEP experiments are presently highly distributed, possibly on
> heterogeneous CPU’s.  In many applications, there is an important need to make information
> available to a large number of other processes in a transparent way. For this purpose the ”RPC-
> like” systems are not suitable, since most of them rely on polling from the client and one-to-
> one connections.  DIM is a very powerful alternative to those systems.  It provides a named
> space for processes to publish information (Publishers) and a very simple API for processes
> willing to use this information (Subscribers).  It fully handles error recovery at the Publisher
> and Subscriber level, without additional software in the application.  DIM is available on a
> large variety of platforms and operating systems with C and C++ bindings. It is presently used
> in several HEP experiments, while it was developed in the DELPHI experiment and is main-
> tained at CERN. We shall present its capabilities and examples of its use in HEP experiments
> in domains ranging from simple data publishing to event transfer, process control or communi-
> cation layer for an Experiment Control Package (SMI++). We shall also present prospectives
> for using it as communications layer for future experiment’s control systems.

- general notes
  * distributed systems oriented
  * alternative to *Remote procedure calls* (RPC)
- techniques
  * publishers/suscribers model
- pros
  * simple API
  * fault-tolerant
  * asynchronous communications
  * one-to-many
- cons
  * *null*
- supervisor notes
  * 

#### (2001) Inter-process Communication Using Different Programming Languages

> A method for inter-process communication between a first process and a second process. The method includes receiving a request from the first process for processing a first process call in a first programming language by the second process, where the second process is configured to execute in a second programming language. The method further includes providing the second process with a procedure to emulate the processing of the first process call, where the procedure is defined in a data structure. Finally, the method includes converting the first process call into a second process call to the procedure using the data structure, executing the second process call using the procedure to obtain a result, and returning the result to the first process.

- general notes
  * patent
  * binary IPC mecanism
  * focuses on Java/C++
- techniques
  * Remote Method Invocation (RMI), Remote Procedure Call (RPC)
- pros
  * IPC through different languages
- cons
  * document hard to understand
- supervisor notes
  * 

#### (2002) NOBLE A Non-Blocking Inter-Process Communication Library

> Many applications on shared memory multi-processor machines can benefit from the exploitation of parallelism offered by nonblocking synchronization. In this paper, we introduce a library called NOBLE, which supports multi-process non-blocking synchronization. NOBLE provides an inter-process communication interface that allows the user to transparently select the synchronization method that is best suited to the current application. The library provides a collection of the most commonly used data types and protocols in a form that allows them to be used by non-experts. We describe the functionality and the implementation of the library functions, and illustrate the library programming style with example programs. We also provide experiments showing that using the library can considerably reduce the run-time of parallel code on shared memory machines.

- general notes
  * adapted to hard RTOS
  * synchronization-oriented
- techniques
  * lock-free and wait-free non-blocking methods
- pros
  * simple interface
  * efficient
  * portable
- cons
  * other libraries may already exist
- supervisor notes
  * 

#### (2004) Fast Platform Independant Inter-process Communication

> A system and method are described for performing data processing using shared memory. In one embodiment, a request to perform a transaction is received at a memory. The memory includes a shared memory to provide a common access to a plurality of components including a plurality of work processes and a plurality of worker nodes. The request is then cached at the memory and later assigned to an available work process for performance.

- general notes
  * patent
  * data processing -related
- techniques
  * shared memory
- pros
  * performance
- cons
  * document hard to understand
- supervisor notes
  * 

#### (2005) Handle Passing Using an Inter-process Communication

> Sharing access to resources using an inter-process communication (“IPC”) provides a connection in which references to resources are passed from a sender to a receiver in a trusted
> third party environment. A sender in possession of a reference to a resource, such as a handle to an object, may initiate the connection with the receiver. In turn, the receiver may accept or refuse the connection, and may further specify the types of resources in which the receiver is interested when accepting through the connection. Sharing access to resources in this manner advantageously insures that only a process that already has access to a resource is able to share that access with another process, and further that only processes that wish to do so will accept such access.

- general notes
  * patent
  * resource sharing between processes
- techniques
  * system resources as objects (Unix : everything as files)
  * duplicate handle from context into other process' context with which share access
  * sender/receiver model
  * medium for sharing resources
- pros
  * secure
- cons
  * document hard to understand
  * performance ?
- supervisor notes
  * 

#### (2005) Inter-process Communication in a Computing Device

> An operating system for a computing device comprises a kernel portion having a publish and subscribe facility for retrieving a property published by a first process and notifying the retrieved property to one or more further processes requesting to subscribe to that property. By providing the
> publish and subscribe mechanism within the operating sys tem kernel, changes to properties can be noticed to subscribers in real time and without the need for dedicated client server mechanisms. The publish and subscribe mechanism may be provided with access control properties established When a property is defined. The mechanism may also be used for a message and message queue facility in the computing device.

- general notes
  * patent
- techniques
  * publisher/suscriber model (client/server too heavy ?)
- pros
  * simple API
- cons
  * document hard to understand
- supervisor notes
  * 

#### (2006) Methodologies to Secure Inter-process Communication Based on Trust

> A system securing inter-process communication (IPC) based on trust includes a user quota mechanism to provide resource management of IPC's. A single user is allowed to allocate a fixed amount of objects less than a system maximum. A trusted IPC mechanism mediates access to IPC objects by employing signed executables signed by a key and containing a list of trusted keys. A trust relationship is used among a set of subjects connected to an IPC to determine whether communication can be carried out between processes. In order for the processes to communicate via an IPC, either they have to trust each other, or a kernel must trust one process and that process must also trust the other process.

- general notes
  * patent
  * IPC based on trust
- techniques
  * Umbrella Process-Based Access Control (PBAC) 
- pros
  * secure
- cons
  * document hard to understand
- supervisor notes
  * 

### Microkernel

#### (1970) The Nucleus of a Multiprogramming System

> This paper describes the philosophy and structure of a multiprogramming system that can be extended with a hierarchy of operating systems to suit diverse requirements of program scheduling and resource allocation. The system nucleus simulates an environment in which program  execution and input/output are handled uniformly as parallel, cooperating processes. A fundamental set of primitives allows the dynamic creation and control of a hierarchy of processes as well as the communication among them.

- general notes
  * defines the notion of kernel and IPC
  * theoretical
- techniques
  * *null*
- pros
  * simple API
- cons
  * old
- supervisor notes
  * 

#### (1974) HYDRA The Kernel of a Multiprogramming Operating System

> This paper describes the design philosophy of HYDRA --the kernel of an operating system for C.mmp, the Carnegie-Mellon Multi-Mini-Processor. This philosophy is realized through the introductiot~ of a generalized notion of "resource," both physical and virtual, called an "object." Mechanisms are presented for dealing with objects, including the creation of new types, specification of new operations applicable to a given type, sharing, and protection of any reference to a given object against improper application of any of the operations defined with respect to that type of object. The mechanisms provide a coherent basis for extension of the system in two directions: the introduction of new facilities, and the creation of highly secure systems.

- general notes
  * **probably too old**
- techniques
  * 
- pros
  * 
- cons
  * old
- supervisor notes
  * 

#### (1994) SPIN - An Extensible Microkernel for Application-specific Operating System Services

> Application domains such as multimedia, databases, and parallel computing, require operating system services with high performance and high functionality. Existing operating systems provide fixed interfaces and implementations to system services and resources. This makes them inappropriate for applications whose resource demands and usage patterns are poorly matched by the services provided. The SPIN operating system enables system services to be defined in an application-specific fashion through an extensible microkernel. It offers applications fine-grained control over a machine's logical and physical resources through run-time adaptation of the system to application requirements.

- general notes
  * 
- techniques
  * 
- pros
  * 
- cons
  * 
- supervisor notes
  * 

#### (1996) Toward Real Microkernels

- general notes
  * magazine
- techniques
  * 
- pros
  * 
- cons
  * 
- supervisor notes
  * 

#### (1999) EMERALDS a small-memory real-time microkernel

> EMERALDS (Extensible Microkernel for Embedded, ReAL-time, Distributed Systems) is a real-time microkernel designed for small-memory embedded applications. These applications must run on slow (15-25MHz) processors with just 32-128 kbytes of memory, either to keep production costs down in mass-produced systems or to keep weight and power consumption low. To be feasible for such applications, the OS must not only be small in size (less than 20 kbytes), but also have low-overhead kernel services. Unlike commercial embedded OSs which rely on carefully-crafted code to achieve efficiency, EMERALDS takes the approach of re-designing the basic OS services of task scheduling, synchronization, communication, and system call mechanism by using characteristics found in small-memory embedded systems, such as small code size and a priori knowledge of task execution and communication patterns. With these new schemes, the overheads of various OS services are reduced 20-40% without compromising any OS functionality.

- general notes
  * 
- techniques
  * 
- pros
  * 
- cons
  * 
- supervisor notes
  * 

#### (2005) A Programmable Microkernel for Real-Time Systems

> We present a new software system architecture for the implementation of hard real-time applications. The core of the system is a microkernel whose reactivity (interrupt handling as in synchronous reactive programs) and proactivity (task scheduling as in traditional RTOSs) are fully programmable. The microkernel, which we implemented on a StrongARM processor, consists of two interacting domain-specific virtual machines, a reactive E (Embedded) machine and a proactive S (Scheduling) machine. The microkernel code (or microcode) that runs on the microkernel is partitioned into E and S code. E code manages the interaction of the system with the physical environment: the execution of E code is triggered by environment interrupts, which signal external events such as the arrival of a message or sensor value, and it releases application tasks to the S machine. S code manages the interaction of the system with the processor: the execution of S code is triggered by hardware interrupts, which signal internal events such as the completion of a task or time slice, and it dispatches application tasks to the CPU, possibly preempting a running task. This partition of the system orthogonalizes the two main concerns of real-time implementations: E code refers to environment time and thus defines the reactivity of the system in a hardware- and scheduler-independent fashion; S code refers to CPU time and defines a system scheduler. If both time lines can be reconciled, then the code is called time safe; violations of time safety are handled again in a programmable way, by run-time exceptions. The separation of E from S code permits the independent programming, verification, optimization, composition, dynamic adaptation, and reuse of both reaction and scheduling mechanisms. Our measurements show that the system overhead is very acceptable even for large sets of task, generally in the 0.2--0.3% range.

- general notes
  * 
- techniques
  * 
- pros
  * 
- cons
  * 
- supervisor notes
  * 

#### (2014) CoMik A Predictable and Cycle-Accurately Composable Real-Time Microkernel

> The functionality of embedded systems is ever increasing. This has lead to mixed time-criticality systems, where applications with a variety of real-time requirements co-exist on the same platform and share resources. Due to inter-application interference, verifying the real-time requirements of such systems is generally non trivial. In this paper, we present the CoMik microkernel that provides temporally predictable and composable processor virtualisation. CoMik's virtual processors are cycle-accurately composable, i.e. their timing cannot affect the timing of co-existing virtual processors by even a single cycle. Real-time applications executing on dedicated virtual processors can therefore be verified and executed in isolation, simplifying the verification of mixed time-criticality systems. We demonstrate these properties through experimentation on an FPGA prototyped hardware platform.

- general notes
  * 
- techniques
  * 
- pros
  * 
- cons
  * 
- supervisor notes
  * 

### RTOS case studies

...

### RTOS general review

...

### RTOS techniques

...

### Rust

...

### Scheduling

...

### Virtual memory management

...

## Optional features

### Asynchronous IO

...

### Bootloader

...

### Filesystem

...


### VxWorks

...

http://www.cs.cornell.edu/Info/Projects/Isis/


# Link, search for consens and disagreement - TODO

...

P/S model
* http://www.cs.ru.nl/~pieter/oss/manyfaces.pdf
* http://blog.nettedautomation.com/2011/01/what-are-clientserver-and.html
* Birman, K. and Joseph, T. "Exploiting virtual synchrony in distributed systems" in Proceedings of the eleventh ACM Symposium on Operating systems principles (SOSP '87), 1987. pp. 123–138.

C/S model
* https://en.wikipedia.org/wiki/Client%E2%80%93server_model

# Plan out how you will present this to the reader - TODO

...

# Good examples

http://campusmoodle.rgu.ac.uk/pluginfile.php/4424058/mod_resource/content/0/Pretty-Good-LitReview.pdf
http://campusmoodle.rgu.ac.uk/pluginfile.php/4424060/mod_resource/content/0/Good_LitReview.pdf
http://campusmoodle.rgu.ac.uk/pluginfile.php/4424059/mod_resource/content/0/Good_Litreview2.pdf