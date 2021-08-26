
# Building a CPU using Kaze

Kaze is a library which allows users to describe hardware like
a hardware descriptor language, as part of Rust source code. In 
this book all Kaze's features are described by guiding you through 
an example of a project in Kaze: building a CPU. 

CPU, as you might already know, stands for "Central Processing Unit" and 
is the heart of any computer. You might even say that the CPU *is* the computer
and any other parts are just peripherals. 

This guide will be split into a number of chapter, also displayed in the sidebar.
We will start with making some simple logic gates in Kaze, which will then be
expanded to an Arithmetic and Logic Unit (ALU). After this we will talk about registers
and memory, and how these, together with the ALU form a data loop. Next up will be 
control logic and conditional execution. More chapters may be added in the future such
as caching, speculative execution and different computer architectures.  
