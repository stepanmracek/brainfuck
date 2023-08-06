import sys
from dataclasses import dataclass
from numba import njit

@dataclass
class Program:
    tokens: list[str]
    opening_to_closing: dict[int, int]
    closing_to_opening: dict[int, int]

def parse(source: str):
    tokens: list[str] = []
    opened_brackets: list[int] = []
    matching_brackets: list[tuple[int, int]] = []
    opening_to_closing: dict[int, int] = {}
    closing_to_opening: dict[int, int] = {}

    source = (token for token in source if token in "<>+-[].")
    for i, token in enumerate(source):
        if token in "<>+-.":
            tokens.append(token)
        elif token == "[":
            opened_brackets.append(i)
            tokens.append(token)
        elif token == "]":
            matching_brackets.append((opened_brackets.pop(), i))
            tokens.append(token)
    tokens.append('x');

    for opening, closing in matching_brackets:
        opening_to_closing[opening] = closing
        closing_to_opening[closing] = opening;
        
    return Program(
        tokens=tokens,
        opening_to_closing=opening_to_closing,
        closing_to_opening=closing_to_opening,
    )


#@njit
def interpret(
    tokens: list[str],
    opening_to_closing: list[int],
    closing_to_opening: list[int]
):
    memory = [0] * 65535
    instruction_pointer = 0
    data_pointer = 0

    while True:
        token = tokens[instruction_pointer]
        if token == '+':
            memory[data_pointer] = (memory[data_pointer] + 1) % 256
            instruction_pointer += 1
        elif token == '-':
            memory[data_pointer] = (memory[data_pointer] - 1) % 256
            instruction_pointer += 1
        elif token == '>':
            data_pointer += 1
            instruction_pointer += 1
        elif token == '<':
            data_pointer -= 1
            instruction_pointer += 1
        elif token == '[':
            if memory[data_pointer] == 0:
                instruction_pointer = opening_to_closing[instruction_pointer] + 1
            else:
                instruction_pointer += 1
        elif token == ']':
            if memory[data_pointer] == 0:
                instruction_pointer += 1
            else:
                instruction_pointer = closing_to_opening[instruction_pointer] + 1
        elif token == ".":
            print(chr(memory[data_pointer]), flush=True, end='')
            #sys.stdout.write(chr(memory[data_pointer]))
            instruction_pointer += 1
        elif token == 'x':
            break

def dict_to_list(source: dict[int, int]):
    max_key = max(source) + 1
    result = [0]*max_key
    for k,v in source.items():
        result[k] = v
    return result

def main():
    with open(sys.argv[1], "rt") as f:
        source = f.read()
    
    program = parse(source)
    interpret(
        program.tokens,
        opening_to_closing=dict_to_list(program.opening_to_closing),
        closing_to_opening=dict_to_list(program.closing_to_opening)
    )

if __name__ == "__main__":
    main()
