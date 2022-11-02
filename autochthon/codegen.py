#!/usr/bin/env python3

import sys
from enum import Enum

class Instruction:
    def __init__(self, name: str, opcode: int, operands: list[str]):
        self.name = name
        self.opcode = opcode
        self.operands = list(map(Operand.from_string, operands))

    def __str__(self):
        description = f'{self.name}[{self.opcode}]'
        if self.operands:
            description += f'({", ".join(map(str, self.operands))})'
        return description

    def size_in_bytes(self):
        # 1 byte for the opcode
        return 1 + sum(o.size_in_bytes() for o in self.operands)

class Operand(Enum):
    Word = 1
    Register = 2
    Address = 3

    @staticmethod
    def from_string(val: str):
        if val == 'word':
            return Operand.Word
        if val == 'register':
            return Operand.Register
        if val == 'address':
            return Operand.Address
        raise AssertionError(f'invalid operand type: {val}')

    def size_in_bytes(self):
        if self == Operand.Word:
            return 4
        if self == Operand.Register:
            return 1
        if self == Operand.Address:
            return 4
        raise AssertionError(f'Invalid Operand enum state: {self}')


def main(args: list[str]):
    validate_args(args)
    raw_lines = get_lines_from_file(args[2])
    processed = preprocess_instructions(raw_lines)
    instructions = create_instruction_list(processed)
    print(list(map(str, instructions)))
    generate_rust_vm_code(list(instructions))

def validate_args(args: list[str]):
    assert len(args) > 0, 'missing program name from command line arguments'
    if len(args) != 3:
        print(f'usage: {args[0]} <generation-target> </path/to/isa.txt>')
        sys.exit(1)

def get_lines_from_file(filename: str) -> list[str]:
    try:
        with open(filename) as f:
            return f.readlines()
    except Exception as e:
        raise AssertionError(f'could not open file: {filename}') from e

def preprocess_instructions(lines: list[str]) -> list[str]:
    def valid_line(l: str) -> bool:
        # Comments and empty lines
        l = l.strip()
        return l and not l.startswith('#')
    def clean_line(l: str) -> str:
        return l.strip().replace(' ', '')
    return list(map(clean_line, filter(valid_line, lines)))

def create_instruction_list(processed_lines: list[str]) -> list[Instruction]:
    opcode = 0
    instructions = []
    for line in processed_lines:
        name, operands_text = split_instruction(line)
        split_operands = (w for w in operands_text.split(',') if w)
        instructions.append(Instruction(name, opcode, split_operands))
        opcode += 1
    return instructions

def split_instruction(line: str) -> (str, list[str]):
    # 'LoadByte(address,register)' -> ('LoadByte', 'address,register')
    open_paren = line.index('(')
    close_paren = line.index(')')
    return (line[0:open_paren], line[open_paren + 1:close_paren])


def generate_rust_vm_code(instructions: list[Instruction]):
    generate_instructions_enum(instructions)
    generate_instructions_impl(instructions)


def tab(n: int) -> str:
    return ' ' * (n * 4)

def generate_instructions_enum(instructions: list[Instruction]):
    print('#[derive(Debug)]')
    print('enum Instruction {')
    for instr in instructions:
        print(f'\t{instr.name}({", ".join(map(rust_type_from_operand, instr.operands))}),')
    print('}\n')

def generate_instructions_impl(instructions: list[Instruction]):
    print('impl Instruction {')
    generate_insruction_size_fn(instructions)
    generate_decode_fn(instructions)
    print('}')

def generate_insruction_size_fn(instructions: list[Instruction]):
    print('\tfn size(&self) -> i8 {')
    print('\t\tmatch self {')
    for instr in instructions:
        print(f'\t\t\t{size_match_arm_from_instruction(instr)}')
    print('\t\t}')
    print('\t}')

def generate_decode_fn(instructions: list[Instruction]):
    print(f'{tab(1)}fn decode(bytes: &[u8]) -> Option<Self> {{')
    print(f'{tab(2)}match bytes {{')
    for instr in instructions:
        print(f'{tab(3)}{decode_match_arm_from_instruction(instr)}')
    print(f'{tab(3)}_ => None')
    print(f'{tab(2)}}}')
    print(f'{tab(1)}}}')

def rust_type_from_operand(op: Operand) -> str:
    if op == Operand.Word or op == Operand.Address:
        return 'i32'
    else:
        return 'u8'

def size_match_arm_from_instruction(instr: Instruction) -> str:
    op_underscores = ', '.join(map(lambda o: '_', instr.operands))
    return f'Self::{instr.name}({op_underscores}) => {instr.size_in_bytes()},'

def decode_match_arm_from_instruction(instr: Instruction) -> str:
    initializers = []
    binding_idx = 0

    def binding_list(start: int, length: int, deref: bool) -> str:
        return ', '.join(f'{"*" if deref else ""}b{i}' for i in range(start, start + length))

    for op in instr.operands:
        if op == Operand.Register:
            initializers.append(f'*b{binding_idx}')
            binding_idx += 1
        else:
            initializers.append(f'convert({binding_list(binding_idx, 4, True)})')
            binding_idx += 4

    bindings = binding_list(0, binding_idx, False) + ', ' if binding_idx != 0 else ''
    return f'[{instr.opcode}, {bindings} ..] => Some(Self::{instr.name}({", ".join(initializers)})),'


if __name__ == '__main__':
    main(sys.argv)
