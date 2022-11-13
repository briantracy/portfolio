
# Autochthon Virtual Machine

The `avm` project is a collection of a few components:

1. A simple RISC instruction set
2. A binary representation of programs and data (symbols) for this instruction set
3. A virtual machine to interpret these binaries
4. An assembler to make writing binaries easier
5. A linker to join together many binaries to create more interesting programs

# Goals


# Implementation

Each program component of the system is written in a different high level language.

The instruction set (`isa.txt`) is defined in a text file to allow for easy modification and to reduce
duplication among components.

The virtual machine (`avm/`) is written in Rust.

The linker (`linker/`) is written in C++.

The assembler (`asm/`) is written in OCaml.


