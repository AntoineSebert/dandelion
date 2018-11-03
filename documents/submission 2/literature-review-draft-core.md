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

#### (2009) Survey and performance evaluation of real-time operating systems for small microcontrollers

> Real-time operating systems have gained popularity in microcontroller- and processor-based embedded system design. This article discusses differences between RTOSs and generic operating systems, the advantages and disadvantages of using RTOSs for small microcontroller system development, and the benchmarking methods used for RTOSs. Benchmarking results for four RTOSs show no clear winner, with each RTOS performing better on some criteria than others.

- general notes
  * benchmark
  * microcontrollers
  * significative (15 RTOS)
- techniques
  * *null*
- pros
  * detailed analysis
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

> Real-time operating systems (RTOSs) provide basic support for scheduling, resource management, synchronization, communication, precise timing, and I/O. RTOSs have evolved from single-use specialized systems to a wide variety of more general-purpose operating systems (such as real-time variants of Linux). We have also seen an evolution from RTOSs which are completely predictable and support safety-critical applications to those which support soft real-time applications. Such support includes the concept of quality of service (QoS) for open real-time systems, often applied to multimedia applications as well as large, complex distributed real-time systems. Researchers in real- time operating system have developed new ideas and paradigms that enhance traditional operating systems to be more efficient and predictable. Some of these ideas are now found in traditional operating systems and many other ideas are found in the wide variety of RTOS on the market today. The RTOS market includes many proprietary kernels, composition-based kernels, and real-time versions of popular OSs such as Linux and Windows-NT. Many industry standards have been influenced by RTOS research including POSIX real-time extensions, Real-Time Specification for Java, OSEK (automotive RTOS standard), Ada83 and Ada95. This paper provides an overview of the architectures, principles, paradigms, and new ideas developed in RTOS research over the past 20 years. The paper concentrates on research done within the context of complete RTOSs. Note that much more research on RTOSs has been accomplished and published as specific aspects on RTOS. For example, real-time synchronization and memory management research has many exciting results. Also, many ideas found in the companion paper on real-time scheduling can be found in various RTOSs as well.

- general notes
  * general presentation
- techniques
  * *null*
- pros
  * present research RTOS
  * present paradigms
- cons
  * *null*
- supervisor notes
  * 

#### (2012) Real-Time Operating Systems

- general notes
  * lecture slides
  * very useful
- techniques
  * *null*
- pros
  * very detailed
  * gives math
- cons
  * *null*
- supervisor notes
  * 

#### (2014) Real-Time Operating System

> Real-time systems play a considerable role in our society, and they cover a spectrum from the very simple to the very complex. Examples of current real-time systems include the control of domestic appliances like washing machines and televisions, the control of automobile engines, telecommunication switching systems, military command and control systems, industrial process control, flight control systems, and space shuttle and aircraft avionics. All of these involve gathering data from the environment, processing of gathered data, and providing timely response. A concept of time is the distinguishing issue between real-time and non-real-time systems. When a usual design goal for non- real-time systems is to maximize system's throughput, the goal for real-time system design is to guarantee, that all tasks are processed within a given time. The taxonomy of time introduces special aspects for real-time system research. Real-time operating systems are an integral part of real-time systems. Future systems will be much larger, more widely distributed, and will be expected to perform a constantly changing set of duties in dynamic environments. This also sets more requirements for future real-time operating systems. This seminar has the humble aim to convey the main ideas on Real Time System and Real Time Operating System design and implementation.

- general notes
  * general presentation
- techniques
  * *null*
- pros
  * schemas
  * very detailed
- cons
  * *null*
- supervisor notes
  * 

### RTOS techniques

#### (2001) Worst-Case Execution Time Analysis of the RTEMS Real-Time Operating System

> An important issue in building operating systems for hard real-time applications is to compute the worst-case execution times (WCET) of the operating system activities. Traditionally, this has been achieved by an exhaustive testing of the operating system, with a careful attention on the testing conditions to reproduce the worst-case execution scenario. In this paper we explore the alternative approach of using static analysis to predict off-line the WCET of the system calls of a real-time kernel, the RTEMS kernel. We give qualitative and quantitative results on the analysis of RTEMS, and draw some conclusions on the extent to which static analysis can be used on operating system code.

- general notes
  * execution time estimation
- techniques
  * static and dynamic analysis
- pros
  * cover main performance concerns
- cons
  * focused on a RTOS : RTEMS
  * maybe highly dependant on other elements (HEPTANE, hardware)
- supervisor notes
  * 

#### (2003) System-Level Power-Aware Design Techniques in Real-Time Systems

> Power and energy consumption has recently become an important issue and consequently, power-aware techniques are being devised at all levels of system design; from the circuit and device level, to the architectural, compiler, operating system, and networking layers. In this paper, we concentrate on power-aware design techniques for real-time systems. While the main focus is on hard real-time, soft real-time systems are considered as well. We start with the motivation for focusing on these systems and provide a brief discussion on power and energy objectives. We then follow with a survey of current research on a layer-by-layer basis. We conclude with illustrative examples and open research challenges. This paper provides an overview of power-aware techniques for the real-time system engineer as well as an up-to-date reference list for the researcher.

- general notes
  * **maybe irrelevant** (but the thematic is interesting and deserves attention imo)
  * CMOS oriented
  * embedded OS oriented
- techniques
  * energy consumption formulas
- pros
  * compiler level, OS level, network level
  * good overall quality
- cons
  * *null*
- supervisor notes
  * 

#### (2006) An Interface Algebra for Real-Time Components

> We present an assume-guarantee interface algebra for real-time components. In our formalism a component implements a set of task sequences that share a resource. A component interface consists of an arrival rate function and a latency for each task sequence, and a capacity function for the shared resource. The interface specifies that the component guarantees certain task latencies depending on assumptions about task arrival rates and allocated resource capacities. Our algebra defines compatibility and refinement relations on interfaces. Interface compatibility can be checked on partial designs, even when some component interfaces are yet unknown. In this case interface composition computes as new assumptions the weakest constraints on the unknown components that are necessary to satisfy the specified guarantees. Interface refinement is defined in a way that ensures that compatible interfaces can be refined and implemented independently. Our algebra thus formalizes an interface-based design methodology that supports both the incremental addition of new components and the independent stepwise refinement of existing components. We demonstrate the flexibility and efficiency of the framework through simulation experiments.

- general notes
  * embedded oriented
  * component based RTOS
  * theoratical
- techniques
  * *null*
- pros
  * detailed
  * lots of formulas
  * maybe very useful
- cons
  * lots of formulas
  * cryptic
- supervisor notes
  * 

#### (2006) Predictable Interrupt Management for Real Time Kernels over Conventional PC Hardware

> In this paper we analyze the traditional model of interrupt management and its incapacity to incorporate reliability and the temporal predictability demanded on real-time systems. As a result of this analysis, we propose a model that integrates interrupts and tasks handling. We make a schedulability analysis to evaluate and distinguish the circumstances under which this integrated model improves the traditional model. The design of a flexible and portable kernel interrupt subsystem for this integrated model is presented. In addition, we present the rationale for the implementation of our design over conventional PC interrupt hardware and the analysis of its overhead. Finally, experimental results are conducted to demonstrate the deterministic behavior of our integrated model and to quantify its overhead.

- general notes
  * interruptions
  * embedded RTOS oriented
- techniques
  * unified mechanism of synchronization and scheduling for hardware and software interrupts
  * two modules for each interrupt type
- pros
  * highlights problems of classical solution
  * schedulability analysis
  * interrupts priorities from 0 to 255
- cons
  * cannot be used in many existing hardware platforms
- supervisor notes
  * 

### Rust

#### (2014) The Rust Language

> Rust is a new programming language for developing reliable and efficient systems. It is designed to support concurrency and parallelism in building applications and libraries that take full advantage of modern hardware. Rust's static type system is safe and expressive and provides strong guarantees about isolation, concurrency, and memory safety.
Rust also offers a clear performance model, making it easier to predict and reason about program efficiency. One important way it accomplishes this is by allowing fine-grained control over memory representations, with direct support for stack allocation and contiguous record storage. The language balances such controls with the absolute requirement for safety: Rust's type system and runtime guarantee the absence of data races, buffer overflows, stack overflows, and accesses to uninitialized or deallocated memory.

- general notes
  * describes the language
- techniques
  * ownership/borrow
  * lifetime-polymorphism
- pros
  * concurrency
  * parallelism
  * safe type system
  * process isolation
  * memory safety
  * performance
- cons
  * young
- supervisor notes
  * 

#### (2016) Engineering the Servo Web Browser Engine using Rust

> All modern web browsers --- Internet Explorer, Firefox, Chrome, Opera, and Safari --- have a core rendering engine written in C++. This language choice was made because it affords the systems programmer complete control of the underlying hardware features and memory in use, and it provides a transparent compilation model. Unfortunately, this language is complex (especially to new contributors!), challenging to write correct parallel code in, and highly susceptible to memory safety issues that potentially lead to security holes.
Servo is a project started at Mozilla Research to build a new web browser engine that preserves the capabilities of these other browser engines but also both takes advantage of the recent trends in parallel hardware and is more memory-safe. We use a new language, Rust, that provides us a similar level of control of the underlying system to C++ but which statically prevents many memory safety issues and provides direct support for parallelism and concurrency.
In this paper, we show how a language with an advanced type system can address many of the most common security issues and software engineering challenges in other browser engines, while still producing code that has the same performance and memory profile. This language is also quite accessible to new open source contributors and employees, even those without a background in C++ or systems programming. We also outline several pitfalls encountered along the way and describe some potential areas for future improvement.

- general notes
  * web browser engine oriented
  * language overview
- techniques
  * affine type system
  * monomorphization (compilation strategy)
  * session types
- pros
  * methods dispatched statically -> eligible for inlining and aggressive optimization
  * data-race free
  * nearly complete interoperability with C code
  * hygienic macro system
- cons
  * lack of interoperability with C++
  * just-in-time code
  * unsafe code correctness
  * incremental computation
- supervisor notes
  * 

### Scheduling

#### (1973) Scheduling Algorithms for Multiprogramming in a Hard-Real-Time Environment

> The problem of multiprogram scheduling on a single processor is studied from the viewpoint of the characteristics peculiar to the program functions that need guaranteed service. It is shown that an optimum fixed priority scheduler possesses an upper bound to processor utilization which may be as low as 70 percent for large task sets. It is also shown that full processor utilization can be achieved by dynamically assigning priorities on the basis of their current deadlines. A combination of these two scheduling techniques is also discussed.

- general notes
  * algorithms
  * important
  * old but gold
- techniques
  * priority-driven algorithms
  * EDF/RM algorithms
  * scheduling algorithm
  	* static (fixed priority scheduling algorithm) : priorities are assigned to tasks once and for all
  	* dynamic : priorities of tasks might change from request to request
  	* mixed : priorities of some of the tasks are fixed yet the priorities of the remaining tasks vary from request to request
- pros
  * algorithm which assigns priorities to tasks in a monotonic relation to their request rates was shown to be optimum among the class of all fixed priority scheduling algorithms
  * dynamic deadline driven scheduling algorithm was then shown to be globally optimum and capable of achieving full processor utilization
  * combination of the two scheduling algorithms was then discussed; this appears to provide most of the benefits of the deadline driven scheduling algorithm
- cons
  * several assumptions
    * (A1) The requests for all tasks for which hard deadlines exist are periodic, with
  	constant interval between requests.
    * (A2) Deadlines consist of run-ability constraints only--i.e, each task must be
  	completed before the next request for it occurs.
    * (A3) The tasks are independent in that requests for a certain task do not depend
  	on the initiation or the completion of requests for other tasks.
    * (A4) Run-time for each task is constant for that task and does not vary with
  	time. Run-time here refers to the time wbich is taken by a processor to execute the
  	task without interruption.
    * (A5) Any nonperiodic tasks in the system are special; they are initialization or
  	failure-recovery routines; they displace periodic tasks while they themselves are
  	being run, and do not themselves have hard, critical deadlines.
- supervisor notes
  * 

#### (2002) Feedback Control Real-Time Scheduling Framework Modeling and Algorithms

> This paper presents a feedback control real-time scheduling (FCS) framework for adaptive real-time systems. An advantage of the FCS framework is its use of feedback control theory (rather than ad hoc solutions) as a scientific underpinning. We apply a control theory based methodology to systematically design FCS algorithms to satisfy the transient and steady state performance specifications of real-time systems. In particular, we establish dynamic models of real-time systems and develop performance analyses of FCS algorithms, which are major challenges and key steps for the design of control theory based adaptive real-time systems. We also present a FCS architecture that allows plug-ins of different real-time scheduling policies and QoS optimization algorithms. Based on our framework, we identify different categories of real-time applications where different FCS algorithms should be applied. Performance evaluation results demonstrate that our analytically tuned FCS algorithms provide robust transient and steady state performance guarantees for periodic and aperiodic tasks even when the task execution times vary by as much as 100% from the initial estimate.

- general notes
  * feedback control real-time scheduling (FCS) framework
- techniques
  * two dynamic scheduling categories : in resource sufficient environment and resource insufficient environment
  * resource sufficient environment : guaranteed that all tasks are a priori schedulable
  * FCS algorithms
  	* FC-U (feedback utilization control)
  	* FC-M (feedback miss ratio control)
  	* FC-UM (integrated utilization/miss ratio control)
  * framework’s elements
  	* scheduling architecture mapping feedback control structure to adaptive resource scheduling
  	* a set of performance specifications and metrics to characterize both transient and steady state performance
  	* control theory based design and methodology for resource scheduling algorithms
- pros
  * allows developer to systematically design adaptive real-time systems to achieve desired performance guarantees in unpredictable environment
  * stability
  * system miss ratio and utilization remain close to the corresponding performance reference
  * achieve satisfactory settling time and zero overshoot under the condition of equation in transient state
  * periodic and aperiodic tasks
- cons
  * *null*
- supervisor notes
  * 

#### (2004) A Categorization of Real-time Multiprocessor Scheduling Problems and Algorithms

> Real-time multiprocessor systems are now commonplace. Designs range from single-chip architectures, with a modest number of processors, to large-scale signal-processing systems, such as synthetic-aperture radar systems. For uniprocessor systems, the problem of ensuring that deadline constraints are met has been widely studied: effective scheduling algorithms that take into account the many complexities that arise in real systems (e.g., synchronization costs, system overheads, etc.) are well understood. In contrast, researchers are just beginning to understand the trade-offs that exist in multiprocessor systems. In this chapter we analyze the trade-offs involved in scheduling independent, periodic real-time tasks on a multiprocessor.

- general notes
  * theoretical
  * exhaustive
- techniques
  * scheduling periodic task systems on multiprocessors
  * partitioning
    * each task is assigned to a single processor, on which each of its jobs will execute, and processors are scheduled independently
    * main advantage of partitioning approaches is that they reduce a multiprocessor scheduling problem to a set of uniprocessor ones
    *  finding an optimal assignment of tasks to processors is a bin-packing problem, which is NP-hard in the strong sense. Thus, tasks are usually partitioned using non-optimal heuristics
    * second, as shown later, task systems exist that are schedulable if and 2 only if tasks are not partitioned
  * global scheduling 
    * all eligible tasks are stored in a single priority-ordered queue; the global scheduler selects for execution the highest priority tasks from this queue
    * using this approach with optimal uniprocessor scheduling algorithms, such as RM and EDF may result in arbitrarily low processor utilization in multiprocessor systems
    * recent research on proportionate fair (Pfair) scheduling has shown considerable promise in that it has produced the only known optimal method for scheduling periodic tasks on multiprocessors
  * middle
    * each job is assigned to a single processor, while a task is allowed to migrate. In other words, inter-processor task migration is permitted only at job boundaries
  * complexity of the priority scheme. Along this dimension, scheduling disciplines are categorized according to whether task priorities are
    * static (RM)
    * dynamic but fixed within a job (EDF)
    * fully dynamic (LLF)
  * The degree of migration allowed. Along this dimension, disciplines are ranked as follows
    * no migration
    * migration allowed, but only at job boundaries
    * unrestricted migration
  * work-conserving scheduling algorithms
    * at each instant, a priority is associated with each active job, and the highest-priority jobs that are eligible to execute are selected for execution upon the available processors
    * job is said to be active at time instant t in a given schedule if
      * it has arrived at or prior to time t
      * its deadline occurs after time t
      * it has not yet completed execution
  * scheduling algorithms taxonomy
  	* Migration-based classification
  		* Interprocessor migration has traditionally been forbidden in real-time systems for the following reasons:
			* In many systems, the cost associated with each migration can be prohibitive
			* Until recently, traditional real-time scheduling theory lacked the techniques, tools, and results to permit a detailed analysis of systems that allow migration. Hence, partitioning has been the preferred approach due largely to the non-existence of viable alternative approaches.
		* No migration (partitioned) : set of tasks is partitioned into as many disjoint subsets as there are processors available, and each such subset is associated with a unique processor. All jobs generated by the tasks in a subset must execute only upon the corresponding processor.
		* Restricted migration : each job must execute entirely upon a single processor. However, different jobs of the same task may execute upon different processors. Thus, the runtime context of each job needs to be maintained upon only one processor; however, the task-level context may be migrated
		* Full migration : No restrictions are placed upon interprocessor migration
	* Priority-based classification
		* Static priorities : unique priority is associated with each task, and all jobs generated by a task have the priority associated with that task. Thus, if task T1 has higher priority than task T2, then whenever both have active jobs, T1’s job will have priority over T2’s job (RM)
		* Job-level dynamic priorities : For every pair of jobs Ji and Jj, if Ji has higher priority than Jj at some instant in time, then Ji always has higher priority than Jj (EDF)
		* Unrestricted dynamic priorities : No restrictions are placed on the priorities that may be assigned to jobs, and the relative priority of two jobs may change at any time (LLF)
- pros
  * *null*
- cons
  * *null*
- supervisor notes
  * 

#### (2005) Rate Monotonic vs. EDF Judgment Day

> Since the first results published in 1973 by Liu and Layland on the Rate Monotonic (RM) and Earliest Deadline First (EDF) algorithms, a lot of progress has been made in the schedulability analysis of periodic task sets. Unfortunately, many misconceptions still exist about the properties of these two scheduling methods, which usually tend to favor RM more than EDF. Typical wrong statements often heard in technical conferences and even in research papers claim that RM is easier to analyze than EDF, it introduces less runtime overhead, it is more predictable in overload conditions, and causes less jitter in task execution.
Since the above statements are either wrong, or not precise, it is time to clarify these issues in a systematic fashion, because the use of EDF allows a better exploitation of the available resources and significantly improves system's performance.
This paper compares RM against EDF under several aspects, using existing theoretical results, specific simulation experiments, or simple counterexamples to show that many common beliefs are either false or only restricted to specific situations.

- general notes
  * 
- techniques
  * under EDF, deadlines need to be updated by the kernel at each job activation, because in a periodic task the absolute deadline changes from a job to the other
  * EDF introduces less runtime overhead than RM, when context switches are taken into account
  * number of preemptions that typically occur under RM is much higher than under EDF
  * real advantage of RM with respect to EDF is its simpler implementation in commercial kernels that do not provide explicit support for timing constraints, such as periods and deadlines
  * Other properties typically claimed for RM, such as predictability during overload conditions, or better jitter control, only apply for the highest priority task, and do not hold in general
  * On the other hand, EDF allows a full processor utilization, which implies a more efficient exploitation of computational resources and a much better responsiveness of aperiodic activities. These properties become very important for embedded systems working with limited computational resources, and for multimedia systems, where quality of service is controlled through resource reservation mechanisms that are much more efficient under EDF
  * both RM and EDF are not very well suited to work in overload conditions and to achieve jitter control. To cope with overloads, specific extensions have been proposed in the literature, both for aperiodic (Buttazzo and Stankovic, 1995) and periodic (Koren and Shasha, 1995) load. Also a method for jitter control under EDF has been addressed in Baruah et al. (1999) and can be adopted whenever needed
- pros
  * *null*
- cons
  * *null*
- supervisor notes
  * 

#### (2006) Bounding Preemption Delay within Data Cache Reference Patterns for Real-Time Tasks

> Caches have become invaluable for higher-end architectures to hide, in part, the increasing gap between processor speed and memory access times. While the effect of caches on timing predictability of single real-time tasks has been the focus of much research, bounding the overhead of cache warm-ups after preemptions remains a challenging problem, particularly for data caches. In this paper, we bound the penalty of cache interference for real-time tasks by providing accurate predictions of the data cache behavior across preemptions. For every task, we derive data cache reference patterns for all scalar and non-scalar references. Partial timing of a task is performed up to a preemption point using these patterns. The effects of cache interference are then analyzed using a settheoretic approach, which identifies the number and location of additional misses due to preemption. A feedback mechanism provides the means to interact with the timing analyzer, which subsequently times another interval of a task bounded by the next preemption. Our experimental results demonstrate that it is sufficient to consider the n most expensive preemption points, where n is the maximum possible number of preemptions. Further, it is shown that such accurate modeling of data cache behavior in preemptive systems significantly improves the WCET predictions for a task. To the best of our knowledge, our work of bounding preemption delay for data caches is unprecedented.

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

#### (2006) FSF A Real-Time Scheduling Architecture Framework

> Scheduling theory generally assumes that real-time systems are mostly composed of activities with hard real-time requirements. Many systems are built today by composing different applications or components in the same system, leading to a mixture of many different kinds of requirements with small parts of the system having hard real-time requirements and other larger parts with requirements for more flexible scheduling and for quality of service. Hard real-time scheduling techniques are extremely pessimistic for the latter part of the application, and consequently it is necessary to use techniques that let the system resources be fully utilized to achieve the highest possible quality. This paper presents a framework for a scheduling architecture that provides the ability to compose several applications or components into the system, and to flexibly schedule the available resources while guaranteeing hard real-time requirements. The framework (called FSF) is independent of the underlying implementation, and can run on different underlying scheduling strategies. It is based on establishing service contracts that represent the complex and flexible requirements of the applications, and which are managed by the underlying system to provide the required level of service.

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

#### (2006) Interface-Based Design of Real-Time Systems with Hierarchical Scheduling

> In interface-based design, components are described by a component interface. In contrast to a component description that describes what a component does, a component interface describes how a component can be used, and a well designed component interface provides enough information to decide whether two or more components can work together properly in a system. Real-Time Interfaces expand the idea of interfacebased design to the area of real-time system design, where the term of working together properly refers to questions like: Does the composed system satisfy all requested real-time properties such as delay and throughput constraints? In this work, we extend the theory of Real-Time Interfaces and prove its applicability for the design of systems with hierarchical scheduling. We introduce a component system for interface-based design of systems with mixed FP, RM and EDF scheduling. We then further extend the ability for hierarchic scheduling by introducing server components. The introduced component system with Real-Time Interfaces not only allows interface-based design of complex real-time systems with hierarchical scheduling, but also inherently enables detailed schedulability analysis of such systems.

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

#### (2006) Task Partitioning with Replication upon Heterogeneous Multiprocessor Systems

> The heterogeneous multiprocessor task partitioning with replication problem involves determining a mapping of recurring tasks upon a set consisting of different processing units in such a way that all tasks meet their timing constraints and no two replicas of the same task are assigned to the same processing unit. The replication requirement improves the resilience of the real-time system to a finite number of processor failures. This problem is NP-hard in the strong sense. We develop a Fully Polynomial-Time Approximation Scheme (FPTAS) for this problem.

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

#### (2007) Extending Rate Monotonic Analysis with Exact Cost of Preemptions for Hard

> In this paper we study hard real-time systems composed of independent periodic preemptive tasks where we assume that tasks are scheduled by using Liu & Layland's pioneering model following the rate monotonic analysis (RMA). For such systems, the designer must guarantee that all the deadlines of all the tasks are met, otherwise dramatic consequences occur. Certainly, guaranteeing deadlines is not always achievable because the preemption is approximated when using this analysis, and this approximation may lead to a wrong real-time execution whereas the schedulability analysis concluded that the system was schedulable. To cope with this problem the designer usually allows margins which are difficult to assess, and thus in any case lead to a waste of resources. This paper makes multiple contributions. First, we show that, when considering the cost of the preemption during the analysis, the critical instant does not occur upon simultaneous release of all tasks. Second, we provide a technique which counts the exact number of preemptions of each instance for all the tasks of a given system. Finally, we present an RMA extension which takes into account the exact cost due to preemption in the schedulability analysis rather than an approximation, thus yielding a new and stronger schedulability condition which eliminates the waste of resources since margins are not necessary.

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

#### (2008) Scheduling The Multi-Level Feedback Queue

> In this chapter, we’ll tackle the problem of developing one of the most well-known approaches to scheduling, known as the Multi-level Feed-back Queue (MLFQ). The Multi-level Feedback Queue (MLFQ) scheduler was first described by Corbato et al. in 1962 [C+62] in a system known as the Compatible Time-Sharing System (CTSS), and this work, along with later work on Multics, led the ACM to award Corbato its highest honor, the Turing Award. The scheduler has subsequently been
refined throughout the years to the implementations you will encounter in some modern systems.
The fundamental problem MLFQ tries to address is two-fold. First, it would like to optimize turnaround time, which, as we saw in the previous note, is done by running shorter jobs first; unfortunately, the OS doesn’t generally know how long a job will run for, exactly the knowledge that algorithms like SJF (or STCF) require. Second, MLFQ would like to make a system feel responsive to interactive users (i.e., users sitting and staring at the screen, waiting for a process to finish), and thus minimize response time; unfortunately, algorithms like Round Robin reduce response time but are terrible for turnaround time. Thus, our problem: given that we in general do not know anything about a process, how can we build a scheduler to achieve these goals? How can the scheduler learn, as the system runs, the characteristics of the jobs it is running, and thus make better scheduling decisions?

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

#### (2011) Predictable Scheduling Algorithms and Applications

> Real-time computing plays a crucial role in our society since an increasing number of complex systems rely, in part or completely, on computer control. Examples of applications that require real-time computing include nuclear power plants, railway switching systems, automotive and avionic systems, air traffic control, telecommunications, robotics, and military systems. In the last several years, real-time computing has been required in new applications areas, such as medical equipments, consumer electronics, multimedia systems, flight simulation systems, virtual reality, and interactive games.
Despite this large application domain, most of the current real-time systems are still designed and implemented using low-level programming and empirical techniques, without the support of a scientific methodology. This approach results in a lack of reliability, which in critical applications may cause serious environmental damage or even loss of life.
This book is a basic treatise on real-time computing, with particular emphasis on predictable scheduling algorithms. The main objectives of the book are to introduce the basic concepts of real-time computing, illustrate the most significant results in the field, and provide the basic methodologies for designing predictable computing systems useful in supporting critical control applications.
This book is written for instructional use and is organized to enable readers without a strong knowledge of the subject matter to quickly grasp the material. Technical concepts are clearly defined at the beginning of each chapter, and algorithm descriptions are corroborated through concrete examples, illustrations, and tables.

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

#### (2014) Adaptive Heterogeneous Scheduling for Integrated GPUs

> Many processors today integrate a CPU and GPU on the same die, which allows them to share resources like physical memory and lowers the cost of CPU-GPU communication. As a consequence, programmers can effectively utilize both the CPU and GPU to execute a single application. This paper presents novel adaptive scheduling techniques for integrated CPU-GPU processors. We present two online profiling-based scheduling algorithms: naïve and asymmetric. Our asymmetric scheduling algorithm uses low-overhead online profiling to automatically partition the work of dataparallel kernels between the CPU and GPU without input from application developers. It does profiling on the CPU and GPU in a way that doesn't penalize GPU-centric workloads that run significantly faster on the GPU. It adapts to application characteristics by addressing: 1) load imbalance via irregularity caused by, e.g., data-dependent control flow, 2) different amounts of work on each kernel call, and 3) multiple kernels with different characteristics. Unlike many existing approaches primarily targeting NVIDIA discrete GPUs, our scheduling algorithm does not require offline processing. We evaluate our asymmetric scheduling algorithm on a desktop system with an Intel 4 th Generation Core Processor using a set of sixteen regular and irregular workloads from diverse application areas. On average, our asymmetric scheduling algorithm performs within 3.2% of the maximum throughput with a CPU-and-GPU oracle that always chooses the best work partitioning between the CPU and GPU. These results underscore the feasibility of online profile-based heterogeneous scheduling on integrated CPU-GPU processors.

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

#### (2017) Research on Real - time Scheduling Method of RTAI - Linux Based on EDF Algorithm

> This paper uses Linux4.2.0 and RTAI3.8.13 to form a dual-core solution to study the real-time performance of Linux systems. This paper studies the basic structure of RTAI-Linux kernel. On this basis, the ADBORROW algorithm is used to improve the time slice rotation strategy of EDF dynamic scheduling algorithm. Assign tasks to RTAI-Linux systems based on dual-core ×86 architectures and perform real-time performance testing and analysis of new task scheduling algorithms. The scheduling model can make the real-time performance of RTAI-Linux system be further improved.

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

#### (1981) WSCLock - A Simple and Effective Algorithm for Virtual Memory Management

> A new virtual memory management algorithm WSCLOCK has been synthesized from the local working set (WS) algorithm, the global CLOCK algorithm, and a new load control mechanism for auxiliary memory access. The new algorithm combines the most useful feature of WS—a natural and effective load control that prevents thrashing—with the simplicity and efficiency of CLOCK. Studies are presented to show that the performance of WS and WSCLOCK are equivalent, even if the savings in overhead are ignored.

- general notes
  * algorithm
- techniques
  * combination of two algorithms (WS and CLOCK)
- pros
  * less memory consuming than WS
  * simpler than both
  * less page faults
- cons
  * more processor consuming
- supervisor notes
  * 

#### (1985) The Integration of Virtual Memory Management and Interprocess Communication in Accent

> The integration of virtual memory management and interprocess communication in the Accent network operating system kernel is examined. The design and implementation of the Accent memory management system is discussed and its performance, both on a series of message-oriented benchmarks and in normal operation, is analyzed in detail.

- general notes
  * overview of memory management
- techniques
  * four basic abstractions : message, port, process, memory object
  * memory object can be : permanent disk, temporary disk, physical memory, port
  * asynchronous IPC
  * messages in transit stored in buffers in the kernel space
  * process map for each user process and one for the operating system kernel
  * VP table (virtual to physical address translation table)
  * memory errors handling
- pros
  * simple way to handle messages
- cons
  * total amount of message data in transit is limited
  * performance comparable to that of more traditional operating system designs
- supervisor notes
  * 

#### (1987) Machine-Independent Virtual Memory Management for Paged Uniprocessor and Multiprocessor Architectures

> This paper describes the design and implementation of virtual memory management within the CMU Mach Operating System and the experiences gained by the Mach kernel group in porting that system to a variety of architectures. As of this writing, Mach runs on more than half a dozen uniprocessors and multiprocessors including the VAX family of uniprocessors and multiprocessors, the IBM RT PC, the SUN 3, the Encore MultiMax, the Sequent Balance 21000 and several experimental computers. Although these systems vary considerably in the kind of hardware support for memory management they provide, the machine-dependent portion of Mach virtual memory consists of a single code module and its related header file. This separation of software memory management from hardware support has been accomplished without sacrificing system performance. In addition to improving portability, it makes possible a relatively unbiased examination of the pros and cons of various hardware memory management schemes, especially as they apply to the support of multiprocessors.

- general notes
  * a bit too theoretical
- techniques
  * machine dependent code decoupled from machine independent
- pros
  * describes virtual memory operations
  * describes virtual memory data structures
  * efficient (in 1987)
  * machine independent
- cons
  * issues
- supervisor notes
  * 

#### (1989) Generic Virtual Memory Management for Operating System Kernels

> We discuss the rationale and design of a Generic Memory management Interface, for a family of scalable operating systems. It consists of a general interface for managing virtual memory, independently of the underlying hardware architecture (e.g. paged versus segmented memory), and independently of the operating system kernel in which it is to be integrated. In particular, this interface provides abstractions for support of a single, consistent cache for both mapped objects and explicit I/O, and control of data caching in real memory. Data management policies are delegated to external managers. A portable implementation of the Generic Memory management Interface for paged architectures, the Paged Virtual Memory manager, is detailed. The PVM uses the novel history object technique for efficient deferred copying. The GMI is used by the Chorus Nucleus, in particular to support a distributed version of Unix. Performance measurements compare favorably with other systems.

- general notes
  * generic memory management interface
- techniques
  * context : program’s protected virtual address space, sparsely populated by regions
  * region : contiguous portion of virtual address space
  * region is mapped to a segment through a local cache
  * region may map a whole segment, or may be a window into part of it
  * protection (e.g. read/write/execute, user/system) is associated with each entire region
  * concurrent access to a segment is allowed
  * segment is always accessed via its corresponding cache
  * memory manager implemented underneath
- pros
  * paged and/or segmented memory
  * written in C++
  * allows efficient implementations
- cons
  * written in C++
- supervisor notes
  * 

#### (1998) Pin-down cache A virtual memory management technique for zero-copy communication

> The overhead of copying data through the central processor by a message passing protocol limits data transfer bandwidth. If the network interface directly transfers the user's memory to the network by issuing DMA, such data copies may be eliminated. Since the DMA facility accesses the physical memory address space, user virtual memory must be pinned down to a physical memory location before the message is sent or received. If each message transfer involves pin-down and release kernel primitives, message transfer bandwidth will decrease since those primitives are quite expensive. The authors propose a zero copy message transfer with a pin-down cache technique which reuses the pinned-down area to decrease the number of calls to pin-down and release primitives. The proposed facility has been implemented in the PM low-level communication library on the RWC PC Cluster II, consisting of 64 Pentium Pro 200 MHz CPUs connected by a Myricom Myrinet network, and running NetBSD. The PM achieves 108.8 MBytes/sec for a 100% pin-down cache hit ratio and 78.7 MBytes/sec for all pin-down cache miss. The MPI library has been implemented on top of PM. According to the NAS parallel benchmarks result, an application is still better performance in case that cache miss ratio is very high.

- general notes
  * zero copy message transfer mechanism
  * cluster oriented
- techniques
  * user-level library
  * pinned memory pages in the pin-down area
- pros
  * eliminate data copy between user specific data area and message buffer
- cons
  * *null*
- supervisor notes
  * 