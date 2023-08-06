import sys
from collections import defaultdict


def parse(program: str):
    stack = [[]]
    for token in program:
        if token in "<>+-.":
            stack[-1].append((token, None))
        elif token == "[":
            child = []
            stack[-1].append((token, child))
            stack.append(child)
        elif token == "]":
            stack[-1].append((token, None))
            stack.pop()
    
    assert len(stack) == 1, "unbalanced parenthesis"
    stack[0].append((None, None))
    return stack[0]


def interpret(syntactic_tree: list, memory: dict[int, int], data_pointer: int):
    instruction_pointer = 0
    stack = [[instruction_pointer, syntactic_tree]]

    while True:
        instruction_pointer, code = stack[-1]
        symbol, child = code[instruction_pointer]

        if symbol == '+':
            memory[data_pointer] = (memory[data_pointer] + 1) % 256
            stack[-1][0] += 1
        elif symbol == '-':
            memory[data_pointer] = (memory[data_pointer] - 1) % 256
            stack[-1][0] += 1
        elif symbol == '>':
            data_pointer += 1
            stack[-1][0] += 1
        elif symbol == '<':
            data_pointer -= 1
            stack[-1][0] += 1
        elif symbol == '[':
            if memory[data_pointer] != 0:
                stack.append([0, child])
            else:
                stack[-1][0] += 1
        elif symbol == ']':
            if memory[data_pointer] != 0:
                stack[-1][0] = 0
            else:
                stack.pop()
        elif symbol == ".":
            print(chr(memory[data_pointer]), flush=True, end='')
            stack[-1][0] += 1
        elif symbol == None:
            break



def main():
    with open(sys.argv[1], "rt") as f:
        program = f.read()
    
    syntactic_tree = parse(program)
    #print(syntactic_tree)

    memory = defaultdict(int)
    pointer = 0
    interpret(syntactic_tree, memory, pointer)


if __name__ == "__main__":
    main()