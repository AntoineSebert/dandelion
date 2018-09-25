# Detailed Project Proposal

| First name     | Anthony          |
| -------------- | ---------------- |
| Last name      | Sébert           |
| Student number | 1705851          |
| Supervisor     | Andrei Petrovski |

## Defining your Project

### 1.1 Project title
Real-Time Operating System in Rust

### 1.2 Background
**Help:** Provide the background to your project. This section should highlight the main topics in the area you are going to research. Essentially what is the project about, what has been done before and why is this project important?  ~500 words

------
This project aims the creation of a real-time operating system using the Rust language. It addresses a large area of design techniques and algorithms so as to reach three defined goals  :
- deterministic execution time of the programs (of subparts of them), allowing to manage the processes accurately
- correctness of the time when a result is expected, to meet or miss the deadlines, whom treatment marks the difference between soft and hard RTOS
- predictability of the deadline associated with a set of constraints to define a expected state of the system
Efficient multitasking is a core principle of the system, and hence a particular attention to the scheduling policy is required, as long as fast context switches to reduce the scheduler's overcost.

Such systems are very useful in several sectors, including astronautics, aeronautics, robotics, or embedded systems in general.

Concerning the Rust language, it is often cited as a potential successor of C++. Indeed, if C++ and Rust are quite close, especially regarding the syntax, the low-level orientation and the memory management, this last natively includes concurrency mangement and safe operations on memory (that can be bypassed if needed).

The most important operating system project written in Rust, Redox, is a microkernel Unix-like OS. There are also a few more projects, some of them for educatioal purpose, plus several kernels/microkernels, but no real-time operating system. Anyway, a review of these projects during the system’s design process will be informative.

------

### 1.3 Motivation
**Help:** To whom is this project important? A project must address a question/problem that generates a small piece of new knowledge/solution. This new knowledge/solution must be important to a named group or to a specific client (such as a company, an academic audience, policy makers, people with disabilities) to make it worthwhile carrying out.  This is the **motivation** for your project.  In this section you should address who will benefit from your findings and how they will benefit.  ~300 words

**Example** 1: If you intend to demonstrate that a mobile application that automates class registers at RGU will be more efficient than paper-based registers - the group who would be interested in knowing/applying these findings would be both academic and administrative staff at RGU and they would benefit by time saved and a reduction in their administrative workload.

**Example** 2: You are demonstrating that a particular 3D model design increases realism in 3D environments. The group that would be interested would be games designers or developers of 3D virtual environment applications. The would benefit from producing more realistic environments that could increase sales of their products.

**Example** 3: You have designed a new network topology for IrishOil plc’s new Aberdeen headquarters. The interested group would clearly be IrishOil. They would benefit from easier maintenance and improved security of their computer network.

------
It seems that no real-time operating system has been written using this promising technology, probably because an important part of the companies that produce this kind of software started their business in the 80s and chose to use the relevant languages at this time, such as C/C++ with assembly.

The advantages of Rust over C/C++ could significantly reduce the ... memory, keeping ... flexibility ... heap/stack. Embedded systems are sometimes quite ... (probe in another planet, aircraft computer) ... sensitive to ... 

------

### 1.4 Aim & Objectives
**Help:**  Outline what are the main things your project is going to do and what steps or milestones will be used to achieve this aim.  The Aim is unlikely to change throughout your project; however, the objectives are likely to adapt to your ongoing research and development. In particular it is highly likely that you may wish to split objectives into sub-objectives as work progresses. A good clear set of objectives give you something to evaluate your final project against.

**Example** : For the timetable app outlined above

Aim: To create a functioning attendance application that efficiently automates the taking of class registers.

Objective 1: study existing register system in place at RGU and identify weaknesses

Objective 2: research existing automation technology’s and identify and evaluate those that may be appropriate to taking in class registers

Objective 3: Implement chosen technologies to create prototype application

Objective 4: Conduct user trials to evaluate capabilities of prototype application

Objective 5: Create a refined application incorporating feedback from user trials

…

real-time operations

safety

performance

concurrency

### 1.5 Key Techniques
**Help:** Perform some initial research into the area and outline what techniques you my research in further detail here. The techniques you cover here should include references to the papers where you have sourced the information. The techniques mentioned here are very likely to become the section headers in your literature review.

…
- **Separation into front-end and back-end modules**. The front-end is responsible for presenting the user interface and drawing a screen full of text. The back-end (also known as “core”) holds the file buffers and is responsible for all potentially expensive editing operations.
- **Rust**. The back-end needs to be extremely performant. In particular, it should use little more memory than the buffers being edited. That level of performance is possible in C++, but Rust offers a much more reliable, and in many ways, higher level programming platform.
- **A persistent rope data structure**. Persistent ropes are efficient even for very large files. In addition, they present a simple interface to their clients - conceptually, they're a sequence of characters just like a string, and the client need not be aware of any internal structure.
- **Asynchronous operations**. The editor should never, ever block and prevent the user from getting their work done. For example, autosave will spawn a thread with a snapshot of the current editor buffer (the persistent rope data structure is copy-on-write so this operation is nearly free), which can then proceed to write out to disk at its leisure, while the buffer is still fully editable.
- **Plug-ins over scripting**. Most text editors have an associated scripting language for extending functionality. However, these languages are usually both more arcane and less powerful than “real” languages. The xi editor will communicate with plugins through pipes, letting them be written in any language, and making it easier to integrate with other systems such as version control, deeper static analyzers of code, etc.
- **JSON**. The protocol for front-end / back-end communication, as well as between the back-end and plug-ins, is based on simple JSON messages. I considered binary formats, but the actual improvement in performance would be completely in the noise. Using JSON considerably lowers friction for developing plug-ins, as it’s available out of the box for most modern languages, and there are plenty of the libraries available for the other ones.


### 1.6 Legal, Social, Ethical, Professional and Security issues
**Help:** Here you should  discuss any legal, social, profession and security issues that you believe may occur during the course of your project. It is not acceptable to write none in this box, all projects, regardless of focus will have to address issues in one, or more, of these categories. This is an extremely important part of your honours project to which there is no correct answer,  this section must be fully discussed with your Honours Supervisor.

**Example 1** : In the class register example above – there would be a Legal and Security issue with the gathering and storage of student data. There may be a social constraint as you may be relying on a user to have access to a specific technology. There will need to be consideration of user accessibility.

**Example 2** : A 3D model design may have ethical considerations in its evaluation. What if your model made users feel nauseous. Social constrains may again be access to technology or accessibility issues.

**Example 3** : You network design need to adhere to specific company policies. You would need to consider the possibility that your design could be wrong, compromising the company’s security.

…

### 1.7 Project Plan
**Help:** This is the project plan as to how you will go about achieving the objectives of the project.

**Example**: In the class register example above the research plan may involve:

Collecting and analysing paper-based registers in a given class on five occasions.

Identifying the error rate average on these occasions

Researching existing automation techniques

Designing and implementing a mobile application that automatically records attendance in class.

Deploying the application in the class on five occasions.

Identifying the error rate average of the mobile application on these occasions.

Comparison of data and summary of findings.

...

### 1.8 Ethics Form
**You must include in your signed ethics form in this submission or you will not be able to continue the project.