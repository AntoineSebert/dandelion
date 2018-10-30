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
  * separation of mechanism and policy
  * rejection of strict hierarchical layering
  * object oriented
- pros
  * multiprocessor computer system
  * interesting design philosophies
  * covers main areas
- cons
  * old
  * too generalist (quality/weakness)
- supervisor notes
  * 

#### (1994) SPIN - An Extensible Microkernel for Application-specific Operating System Services

> Application domains such as multimedia, databases, and parallel computing, require operating system services with high performance and high functionality. Existing operating systems provide fixed interfaces and implementations to system services and resources. This makes them inappropriate for applications whose resource demands and usage patterns are poorly matched by the services provided. The SPIN operating system enables system services to be defined in an application-specific fashion through an extensible microkernel. It offers applications fine-grained control over a machine's logical and physical resources through run-time adaptation of the system to application requirements.

- general notes
  * microkernel OS oriented
- techniques
  * install service at kernel level
  * run application code (service) as a kernel module (tightly coupled)
  * application level libray (loosely coupled)
  * extended IPC (message interpretation left to receiver)
- pros
  * adaptability
- cons
  * security (must trust app publishers)
- supervisor notes
  * 

#### (1996) Toward Real Microkernels

> The inefficient, inflexible first generation inspired development of the vastly improved second generation, which may yet support a variety of operating systems.

- general notes
  * magazine
  * overview of microkernels
- techniques
  * *null*
- pros
  * quick concept reminder
- cons
  * *null*
- supervisor notes
  * 

#### (1999) EMERALDS a small-memory real-time microkernel

> EMERALDS (Extensible Microkernel for Embedded, ReAL-time, Distributed Systems) is a real-time microkernel designed for small-memory embedded applications. These applications must run on slow (15-25MHz) processors with just 32-128 kbytes of memory, either to keep production costs down in mass-produced systems or to keep weight and power consumption low. To be feasible for such applications, the OS must not only be small in size (less than 20 kbytes), but also have low-overhead kernel services. Unlike commercial embedded OSs which rely on carefully-crafted code to achieve efficiency, EMERALDS takes the approach of re-designing the basic OS services of task scheduling, synchronization, communication, and system call mechanism by using characteristics found in small-memory embedded systems, such as small code size and a priori knowledge of task execution and communication patterns. With these new schemes, the overheads of various OS services are reduced 20-40% without compromising any OS functionality.

- general notes
  * embedded OS oriented
- techniques
  * strong design constraints (25 Mhz processor, 128kb RAM, 20kb max size)
  * IPC based on message-passing, mailboxes, and shared memory
- pros
  * IoT friendly
  * highly optimized context switching and interrupt handling
  * support for user-level device drivers
- cons
  * constraints does not exist anymore
- supervisor notes
  * 

#### (2005) A Programmable Microkernel for Real-Time Systems

> We present a new software system architecture for the implementation of hard real-time applications. The core of the system is a microkernel whose reactivity (interrupt handling as in synchronous reactive programs) and proactivity (task scheduling as in traditional RTOSs) are fully programmable. The microkernel, which we implemented on a StrongARM processor, consists of two interacting domain-specific virtual machines, a reactive E (Embedded) machine and a proactive S (Scheduling) machine. The microkernel code (or microcode) that runs on the microkernel is partitioned into E and S code. E code manages the interaction of the system with the physical environment: the execution of E code is triggered by environment interrupts, which signal external events such as the arrival of a message or sensor value, and it releases application tasks to the S machine. S code manages the interaction of the system with the processor: the execution of S code is triggered by hardware interrupts, which signal internal events such as the completion of a task or time slice, and it dispatches application tasks to the CPU, possibly preempting a running task. This partition of the system orthogonalizes the two main concerns of real-time implementations: E code refers to environment time and thus defines the reactivity of the system in a hardware- and scheduler-independent fashion; S code refers to CPU time and defines a system scheduler. If both time lines can be reconciled, then the code is called time safe; violations of time safety are handled again in a programmable way, by run-time exceptions. The separation of E from S code permits the independent programming, verification, optimization, composition, dynamic adaptation, and reuse of both reaction and scheduling mechanisms. Our measurements show that the system overhead is very acceptable even for large sets of task, generally in the 0.2-0.3% range.

- general notes
  * microkernel OS
- techniques
  * two interacting domain-specific virtual machines, a reactive E (Embedded) machine and a proactive S (Scheduling) machine
- pros
  * adaptable
  * minimal
- cons
  * *null*
- supervisor notes
  * 

#### (2014) CoMik A Predictable and Cycle-Accurately Composable Real-Time Microkernel

> The functionality of embedded systems is ever increasing. This has lead to mixed time-criticality systems, where applications with a variety of real-time requirements co-exist on the same platform and share resources. Due to inter-application interference, verifying the real-time requirements of such systems is generally non trivial. In this paper, we present the CoMik microkernel that provides temporally predictable and composable processor virtualisation. CoMik's virtual processors are cycle-accurately composable, i.e. their timing cannot affect the timing of co-existing virtual processors by even a single cycle. Real-time applications executing on dedicated virtual processors can therefore be verified and executed in isolation, simplifying the verification of mixed time-criticality systems. We demonstrate these properties through experimentation on an FPGA prototyped hardware platform.

- general notes
  * microkernel RTOS
  * very relevant
- techniques
  * processor virtualization
  * divides the processor into Time Division Multiplexed (TDM) slots
- pros
  * context swapping scheme that ensures that absolutely no temporal interference occurs between partitions, not even a single cycle
- cons
  * *null*
- supervisor notes
  * 

### RTOS case studies

#### (1989) Distributed Fault-Tolerant Real-Time Systems The Mars Approach

> The authors describe the Maintainable Real-Time System, a fault-tolerant distributed system for process control, developed under the Mars project started in 1980 at the Technische Universitat Berlin. They explore the characteristics of distributed real-time systems and then present the Mars approach to real-time process control, its architectural design and implementation, and one of its applications. The authors focus on the maintainability of the Mars architecture, describe the Mars operating system, and discuss timing analysis. The control of a rolling mill that produces metal plates and bars is examined.

- general notes
  * distributed RTOS oriented
  * highly relevant
- techniques
  * information is *valid* if *correct* and *timely*
  * transactions
  * real-time data and archived data
  * global time
- pros
  * fault-tolerant
  * compares *event-driven* and *time-sharing*
  * quite exhaustive
- cons
  * old
- supervisor notes
  * 

#### (1989) The MARUTI Hard Real-Time Operating System

> A description is given of the MARUTI operating system, which is designed to support real-time applications on a variety of hardware systems. The kernel supports objects as primitive entities and provides a communication mechanism that allows transparent distribution in networked systems. Fault tolerance is provided through replication and consistency-control mechanisms. MARUTI supports guaranteed-service scheduling, in which jobs that are accepted by the system are verified to satisfy general time constraints. These time constraints include those that govern interrupt processing, which allows the MARUTI approach to succeed where more rigorous approaches do not. The result is that MARUTI applications can be executed in a predictable, deterministic fashion.

- general notes
  * distributed RTOS
  * modular OS
  * Unix
- techniques
  * guaranteed-service scheduling
- pros
  * hard real-time
  * fault-tolerant
- cons
  * old
- supervisor notes
  * 

#### (1989) The Spring Kernel A New Paradigm for Real-Time Operating Systems

> Next generation real-time systems will require greater flexibility and predictability than is commonly found in today's systems. These future systems include the space station, integrated vision/robotics/AI systems, collections of humans/robots coordinating to achieve common objectives (usually in hazardous environments such as undersea exploration or chemical plants), and various command and control applications. The Spring kernel is a research oriented kernel designed to form the basis of a flexible, hard real-time operating system for such applications. Our approach challenges several basic assumptions upon which most current real-time operating systems are built and subsequently advocates a new paradigm based on the notion of predictability and a method for on-line dynamic guarantees of deadlines. The Spring kernel is being implemented on a network of (68020 based) multiprocessors called SpringNet.

- general notes
  * hard RTOS
  * presents defaults of time-sharing
  * highly relevant
- techniques
  * dynamic scheduling algorithms
  * pre-scheduling tasks analysis
  * dynamic value of task, to be maximized
  * time properties of applications & system itself
- pros
  * flexibility
  * predictability
- cons
  * old
- supervisor notes
  * 

#### (1997) A Linux-based Real-Time Operating System

> This work describes the design, implementation, and possible applications of Real-Time Linux - a hard real-time version of the Linux operating system. In this system, a standard time-sharing OS and a real-time executive run on the same computer. Interrupt controller emulation is used to guarantee a low maximum interrupt latency independently of the base system. The use of a one-shot timer makes it possible to achieve a low task release jitter without compromising throughput. Lock-free FIFO buffers are employed for communication between real-time tasks and Linux processes. User-defined schedulers are allowed as are run-time changes in the scheduling policy.

- general notes
  * hard time RTOS
  * Linux
  * time-sharing OS and a real-time executive at the same time
- techniques
  * interrupt controller emulation (low maximum interrupt latency), platform independant
  * one-shot timer (possible to achieve a low task release jitter without compromising throughput)
  * lock-free FIFO buffers (communication between real-time tasks and Linux processes)
- pros
  * well explained
- cons
  * invents nothing
- supervisor notes
  * 

#### (2009) A Real Time Operating Systems (RTOS) Comparison

> This  article  presents  quantitative  and  qualitative  results  obtained from the analysis of real time operating systems (RTOS). The studied systems were Windows CE, QNX Neutrino, VxWorks, Linux and RTAI-Linux, which are largely used in industrial and academic environments. Windows XP was also analysed, as a reference for conventional non-real-time operating system, since such systems are also commonly and inadvertently used for instrumentation and control purposes. The evaluations include worst case response times for latency, latency jitter and response time.

- general notes
  * quite recent
  * covers most used RTOS
- techniques
  * black-box testing
- pros
  * detailed analysis
  * useful informations on several aspects
  * qualitative and quantitative
- cons
  * *null*
- supervisor notes
  * 

#### (2014) The survey of Real-Time Operating System RTOS

> The paper discusses the literature survey of RTOS (Real Time Operating Systems) and its contributions to the embedded world. RTOS is defined as a system in which the correctness of the system does not depend only on the logical results of computation but also on the time at which the results are produced. It has to perform critical tasks on priority basis keeping the context switching time minimum. It is often associated with few misconceptions & we have tried to throw some light on it. Since last 20 years, RTOS is undergoing continuous evolution and has resulted into development of many commercial RTOS products. We have selected few commercial RTOS of different categories of real-time applications and have discussed its real-time features. A comparison of the commercial RTOSs' is presented. We conclude by discussing the results of the survey and comparing the RTOS based on performance parameters.

- general notes
  * recent
- techniques
  * *null*
- pros
  * qualitative and quantitative
  * explains the concepts
- cons
  * *null*
- supervisor notes
  * 

### RTOS general review

#### (2004) Real-Time Operating Systems

> Real-time operating systems (RTOSs) provide basic support for scheduling, resource management, synchronization, communication, precise timing, and I/O. RTOSs have evolved from single-use specialized systems to a wide variety of more general-purpose operating systems (such as real-time variants of Linux). We have also seen an evolution from RTOSs which are completely predictable and support safety-critical applications to those which support soft real-time applications. Such support includes the concept of quality of service (QoS) for open real-time systems, often applied to multimedia applications as well as large, complex distributed real-time systems. Researchers in real- time operating system have developed new ideas and paradigms that enhance traditional operating systems to be more ef®cient and predictable. Some of these ideas are now found in traditional operating systems and many other ideas are found in the wide variety of RTOS on the market today. The RTOS market includes many proprietary kernels, composition-based kernels, and real-time versions of popular OSs such as Linux and Windows-NT. Many industry standards have been influenced by RTOS research including POSIX real-time extensions, Real-Time Speci®cation for Java, OSEK (automotive RTOS standard), Ada83 and Ada95. This paper provides an overview of the architectures, principles, paradigms, and new ideas developed in RTOS research over the past 20 years. The paper concentrates on research done within the context of complete RTOSs. Note that much more research on RTOSs has been accomplished and published as specific aspects on RTOS. For example, real-time synchronization and memory management research has many exciting results. Also, many ideas found in the companion paper on real-time scheduling can be found in various RTOSs as well.

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

#### (2012) Real-Time Operating Systems

- general notes
  * lecture slides
- techniques
  * 
- pros
  * 
- cons
  * 
- supervisor notes
  * 

#### (2014) Real-Time Operating System

> Real-time systems play a considerable role in our society, and they cover a spectrum from the very simple to the very complex. Examples of current real-time systems include the control of domestic appliances like washing machines and televisions, the control of automobile engines, telecommunication switching systems, military command and control systems, industrial process control, flight control systems, and space shuttle and aircraft avionics. All of these involve gathering data from the environment, processing of gathered data, and providing timely response. A concept of time is the distinguishing issue between real-time and non-real-time systems. When a usual design goal for non- real-time systems is to maximize system's throughput, the goal for real-time system design is to guarantee, that all tasks are processed within a given time. The taxonomy of time introduces special aspects for real-time system research. Real-time operating systems are an integral part of real-time systems. Future systems will be much larger, more widely distributed, and will be expected to perform a constantly changing set of duties in dynamic environments. This also sets more requirements for future real-time operating systems. This seminar has the humble aim to convey the main ideas on Real Time System and Real Time Operating System design and implementation.

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

### RTOS techniques

...

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

### Rust

...

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

### Scheduling

...


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
### Virtual memory management

...

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