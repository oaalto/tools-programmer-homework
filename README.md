# Tools Programmer Homework

Your mission: implement a web service that takes in a set of bytes as input and outputs a Motorola 680x0 processor 
family disassembly of those bytes as output. For example, assuming the following input bytes:

````
e7 48 20 20 21 70 21 
````

The following output (or similar) should be produced:

````
0x00000000 48e7 2020   MOVEM.L D5,A5,-(A7)
0x00000004 7021        MOVEQ #$21, D0
````

You can, based on your own preferences, change the format of the output, but the origin address and bytes used in
disassembly should always be visible. You can deal with unknown opcodes however you deem appropriate
(outputting just the hex bytes is sufficient, for example). You should assume the input begins at address 0.
There are some example binaries in the `test-bin/` directory of the source tree that contain valid 68000 code that
can be used for testing.

Information on the Motorola 68000 instruction encoding can be found, for example in section 2 of
the Motorola 68000 Programmer's Reference Manual, available at 
https://www.nxp.com/docs/en/reference-manual/M68000PRM.pdf

A simple testing scaffold is provided for testing and development convenience. You can `cargo run` the scaffold in 
one terminal to start up the testing web server and then run `cargo test` in another to send a test request to it. 
You can modify the existing test and add new ones as desired.

You can spend as much time as you feel is required for a baseline implementation. The idea is not to find out if you 
can read the PRG and implement instruction decoding - we can assume this is the case. If you can find a way to produce 
proper disassembly from the binary data without implementing a full disassembler, feel free to do so! 

Incomplete solutions are also better than nothing! Don't worry if you don't have time to implement all instructions or
are missing some 68030 specific or undocumented ones! If you find the task to be unexpectedly difficult, don't worry!
If you can provide some initial code and are prepared to explain your implementation idea, that is fine as well.

Don't hesitate to use pseudocode in longer-running sections - 680x0 instruction decoding is simple on the surface but
can have a lot of non-obvious gotchas.
