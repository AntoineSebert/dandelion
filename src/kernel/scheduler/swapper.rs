/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

/*
Yes, you typically want to save all general purpose registers. The reason is that you have to save all registers that the interrupted job uses, so that it can continue later. Note that you need to also save the SSE registers if you enable SSE at some point.

Implementing context switches is a bit tricky because you have to be very careful to not accidentally overwrite registers before saving them. One way to do this is to make the interrupt handler a naked function that does the following:

- push all registers to the stack
- call a function that retuns the stack pointer of the next thread from the thread list
- call a function that puts the current thread (represented by the stack pointer) into the thread list
- switch the stack by updating the rsp register
- pop the saved register values of the thread from stack

You probably want to use a different interrupt stack (like we did for the double fault exception) to avoid the race condition between step 3 and 4.

Note that this is only a high-level overview and only one way of doing it. For the blog I plan to use a different approach but I'm not sure if it works out yet.
*/

//pub fn context_switch(job) {}

//fn push_registers(registers) {}

fn get_next_stack_pointer() {}

fn get_current_task_pointer() {}

fn rsp_update() {}

fn pop_register() {}
