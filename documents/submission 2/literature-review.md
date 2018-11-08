# Literature review

## Introduction

Real-Time Operating Systems (RTOS) is a type of Operating System (OS) that are bound to real-time constraints, often translated as deadlines, and meant to run real-time applications. Real-Time Computing (RTC) brings dedicated concepts to characterize such systems (the *jitter* for example), and divide the RTOS into three categories, depending on their tolerance :
- hard: missing a deadline is considered a failure
- firm: infrequent deadline misses are tolerated, but might deteriorate the system's reliability
- soft: missing a deadline is tolerated but might deteriorate the system's reliability

RTOS encompasses many other computer science domains. In addition to the classics virtual memory management, file system, I/O handling, and IPC, they draw a particular attention to the scheduling algorithm (which is almost systematically preemptive), the processes management and the interrupts. Two main design philosophies currently coexist :
- event-driven: preemption based on event's priorities
- time-sharing: preemption on system clock interrupts and on events

Applications of such systems are multiple, especially in areas where reliability is a key aspect, even with radically different purposes. RTOS are typically found in astronautics, mainframes, aeronautics, robotics, or embedded systems in general (and IoT in particular). They are typically more tolerant to failures and more predictable, and particular care is taken about the implementation and the tests.

However, new possibilities recently arose with the emergence of new tools centered around solving well-defined problems with commonly-used techniques. This brings us to Rust: originally created by Graydon Hoare, who worked at the Mozilla Foundation, it is now developed by the Rust project developers, centered around the language's Github repository. Designed to be fast and reliable, this language is memory safe, uses Resource Acquisition Is Initialization (RAII) to manage memory (avoiding the necessity of a garbage collector), supports polymorphism and natively supports concurrency.

Whereas the majority of current RTOS have been conceived when Rust did not exist, they are written in C++. Hence a RTOS implemented in Rust seems very promising. Very few attempts in this area have been made, and yet RTC is a growing field in the need for a new generation of RTOS aimed to solve arising challenges.

## Body
by **theme** : useful if several trends that can logically be considered separately before being brought together

### Link, search for consens and disagreement - TODO

#### IPC

two major paradigms : synchronous/asynchronous
**synchronous** (*σύν* (with, in company with, together with); *χρόνος* (time))
* requirement : participants must be available at the same time
    * corollaire : synchronization phase at first
* blocking events
* corollaire : messages order is invariant
* conjecture : short-time applications

**asynchronous** (not synchronous)
* requirement : a space to store the data must exist
* non-blocking events
* conjecture : messages order need to be explicited if it is necessary
* corollaire : receiver may not exist (error)

##### link

mostly distributed systems oriented
* (1989) Preserving and Using Context Information in Interprocess Communication
* (1990) On Global-Time and Inter-process Communication
* (1995) The Real-Time PublisherSubscriber Inter-Process Communication
* (2001) DIM, a portable, light weight package for information publishing, data transfer and inter-process communication

internal IPC
* (2001) Inter-process Communication Using Different Programming Languages
* (2002) NOBLE A Non-Blocking Inter-Process Communication Library
* (2005) Handle Passing Using an Inter-process Communication
* (2005) Inter-process Communication in a Computing Device
* (2006) Methodologies to Secure Inter-process Communication Based on Trust

##### consens

pseudosynchronous
* (1989) Preserving and Using Context Information in Interprocess Communication
	* partial ordering

##### disagreement

...

#### Microkernel

##### link

...

##### consens

...

##### disagreement

...

#### RTOS

##### link

...

##### consens

...

##### disagreement

...

#### Rust (vs other languages)

##### link

...

##### consens

...

##### disagreement

...

#### Scheduling

##### link

...

##### consens

...

##### disagreement

...

#### Virtual Memory Management

##### link

...

##### consens

...

##### disagreement

...


## Summary

highlight min gaps and opportunities discovered and proposing ...

------

P/S model
* http://www.cs.ru.nl/~pieter/oss/manyfaces.pdf
* http://blog.nettedautomation.com/2011/01/what-are-clientserver-and.html

C/S model
* https://en.wikipedia.org/wiki/Client%E2%80%93server_model

## Plan out how you will present this to the reader - TODO

...

## Good examples

http://campusmoodle.rgu.ac.uk/pluginfile.php/4424058/mod_resource/content/0/Pretty-Good-LitReview.pdf
http://campusmoodle.rgu.ac.uk/pluginfile.php/4424060/mod_resource/content/0/Good_LitReview.pdf
http://campusmoodle.rgu.ac.uk/pluginfile.php/4424059/mod_resource/content/0/Good_Litreview2.pdf