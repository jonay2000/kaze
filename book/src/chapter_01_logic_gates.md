# chapter 1: Logic gates

Processors are filled with components called logic gates. Logic gates are simple
machines taking one or more inputs and producing an output based on that. Probably 
the simplest is called an inverter (or "not" gate). It has a single input and always
outputs the opposite of whatever it gets in. In Kaze, this can be represented
as follows:

```rust,no_run,noplayground
{{#include ../kaze-examples/src/chapter_01_logic_gates.rs:inverter}}
```

The inverter takes a 1 bit input, and outputs the opposite by using the `!` operator.

Using Kaze, it is now possible to convert this simple module definition to Rust code
simulating the working of this logic gate: 

```rust,no_run,noplayground
{{#include ../kaze-examples/src/chapter_01_logic_gates.rs:inverter_code}}
```

However, this simulating code on its own is quite useless. We want to see the module working!
To do this we instantiate the inverter as part as a new testing module:

```rust,no_run,noplayground

```
