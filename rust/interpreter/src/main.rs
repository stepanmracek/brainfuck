use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};

#[derive(Debug)]
struct Program {
    tokens: Vec<(char, usize)>,
    matching_brackets: HashMap<usize, usize>,
}

fn tokenize(program: &str) -> Vec<(char, usize)> {
    let valit_tokens: HashSet<char> = HashSet::from(['<', '>', '+', '-', '[', ']', '.', ',']);
    let non_compressable_tokens: HashSet<char> = HashSet::from(['[', ']', '.', ',']);

    let mut tokens: Vec<(char, usize)> = vec![];
    let mut last_token = '_';
    let mut count: usize = 0;
    for token in program.chars().filter(|ch| valit_tokens.contains(ch)) {
        if (token != last_token && count > 0) || (non_compressable_tokens.contains(&token)) {
            tokens.push((last_token, count));
            count = 0;
        }
        last_token = token;
        count += 1;
    }
    tokens.push((last_token, count));
    tokens.push(('x', 0));
    tokens
}

fn find_matching_brackets(tokens: &Vec<(char, usize)>) -> HashMap<usize, usize> {
    let mut opened_brackets: Vec<usize> = vec![];
    let mut matching_brackets: HashMap<usize, usize> = HashMap::new();
    for (i, (token, _count)) in tokens.iter().enumerate() {
        match token {
            '[' => {
                opened_brackets.push(i);
            }
            ']' => {
                let opening = opened_brackets.pop().unwrap();
                matching_brackets.insert(opening, i);
                matching_brackets.insert(i, opening);
            }
            _ => {}
        }
    }
    matching_brackets
}

fn parse(program: &str) -> Program {
    let tokens = tokenize(program);
    let matching_brackets = find_matching_brackets(&tokens);

    Program {
        tokens,
        matching_brackets,
    }
}

fn interpret(program: Program) {
    let mut memory: Vec<u8> = vec![0; 65536];
    let mut instruction_pointer: usize = 0;
    let mut data_pointer: usize = 0;

    loop {
        let (token, count) = program.tokens[instruction_pointer];
        if token == '+' {
            memory[data_pointer] = (memory[data_pointer] as usize + count) as u8;
        } else if token == '-' {
            memory[data_pointer] = (memory[data_pointer] as usize - count) as u8;
        } else if token == '>' {
            data_pointer += count;
        } else if token == '<' {
            data_pointer -= count;
        } else if token == '[' && memory[data_pointer] == 0 {
            instruction_pointer = *program.matching_brackets.get(&instruction_pointer).unwrap();
        } else if token == ']' && memory[data_pointer] != 0 {
            instruction_pointer = *program.matching_brackets.get(&instruction_pointer).unwrap();
        } else if token == '.' {
            print!("{}", memory[data_pointer] as char);
            let _ = std::io::stdout().flush();
        } else if token == ',' {
            let mut input: [u8; 1] = [0];
            std::io::stdin().read(&mut input).unwrap();
            memory[data_pointer] = input[0];
        } else if token == 'x' {
            break;
        }
        instruction_pointer += 1;
    }
}

fn main() {
    let file = std::env::args().last().unwrap();
    let source = std::fs::read_to_string(file).unwrap();
    let program = parse(&source);
    interpret(program)
}
