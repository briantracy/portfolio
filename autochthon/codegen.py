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
            return 1
        if self == Operand.Register:
            return 4
        if self == Operand.Address:
            return 4
        raise AssertionError(f'Invalid Operand enum state: {self}')


def main(args: list[str]):

    validate_args(args)
    raw_lines = get_lines_from_file(args[2])
    print(raw_lines)
    processed = preprocess_instructions(raw_lines)
    print(processed)
    instructions = create_instruction_list(processed)
    print(list(map(str, instructions)))

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
    open_paren = line.index('(')
    close_paren = line.index(')')
    return (line[0:open_paren], line[open_paren + 1:close_paren])

if __name__ == '__main__':
    main(sys.argv)
