import sys
from dataclasses import dataclass
from itertools import groupby, chain
from typing import Iterable


@dataclass
class Program:
    tokens: tuple[tuple[str, int],...]
    matching_brackets: dict[int, int]


def tokenize(source: str) -> tuple[tuple[str, int],...]:
    source = (token for token in source if token in "<>+-[].")
    # group only "<>+-", for brackets and IO ("[].") create special key (pos, token) with unique pos which
    # ensures that consequent brackets and IO statements won't be grouped
    source = groupby(enumerate(source), key=lambda token: token[1] if token[1] in "<>+-" else token)
    # convert special brackets and IO tokens (pos, token) to just token
    source = (
        (token[1], 1) if isinstance(token, tuple) else (token, len(list(group)))
        for token, group in source
    )
    # append special term statement
    return tuple(chain(source, (('x', 0),)))


def find_matching_brackets(tokens: tuple[tuple[str, int],...]) -> dict[int, int]:
    opened_brackets: list[int] = []
    matching_brackets: dict[int, int] = {}

    for i, (token, count) in enumerate(tokens):
        if token == "[":
            opened_brackets.append(i)
        elif token == "]":
            j = opened_brackets.pop()
            matching_brackets[i] = j
            matching_brackets[j] = i

    return matching_brackets


def parse(source: str):
    tokens = tokenize(source)
    matching_brackets = find_matching_brackets(tokens)

    return Program(
        tokens=tokens,
        matching_brackets=matching_brackets,
    )


def interpret(program: Program):
    memory = [0] * 65536
    instruction_pointer = 0
    data_pointer = 0

    while True:
        token, count = program.tokens[instruction_pointer]
        if token == '+':
            memory[data_pointer] = (memory[data_pointer] + count) % 256
        elif token == '-':
            memory[data_pointer] = (memory[data_pointer] - count) % 256
        elif token == '>':
            data_pointer += count
        elif token == '<':
            data_pointer -= count
        elif token == '[':
            if memory[data_pointer] == 0:
                instruction_pointer = program.matching_brackets[instruction_pointer]
        elif token == ']':
            if memory[data_pointer] != 0:
                instruction_pointer = program.matching_brackets[instruction_pointer]
        elif token == ".":
            print(chr(memory[data_pointer]), flush=True, end='')
        elif token == 'x':
            break
        instruction_pointer += 1


def main():
    with open(sys.argv[1], "rt") as f:
        source = f.read()
    
    program = parse(source)
    interpret(program)


if __name__ == "__main__":
    main()
