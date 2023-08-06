use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{stdout, Write};

#[derive(Debug)]
struct Program {
    tokens: Vec<(char, usize)>,
    matching_brackets: HashMap<usize, usize>,
}

fn tokenize(program: &str) -> Vec<(char, usize)> {
    let valit_tokens: HashSet<char> = HashSet::from(['<', '>', '+', '-', '[', ']', '.']);
    let non_compressable_tokens: HashSet<char> = HashSet::from(['[', ']', '.']);

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
        let token = program.tokens[instruction_pointer];
        match token {
            ('+', count) => memory[data_pointer] = (memory[data_pointer] as usize + count) as u8,
            ('-', count) => memory[data_pointer] = (memory[data_pointer] as usize - count) as u8,
            ('>', count) => data_pointer += count,
            ('<', count) => data_pointer -= count,
            ('[', _) => {
                if memory[data_pointer] == 0 {
                    instruction_pointer =
                        *program.matching_brackets.get(&instruction_pointer).unwrap();
                }
            }
            (']', _) => {
                if memory[data_pointer] != 0 {
                    instruction_pointer =
                        *program.matching_brackets.get(&instruction_pointer).unwrap();
                }
            }
            ('.', _) => {
                print!("{}", memory[data_pointer] as char);
                let _ = stdout().flush();
            }
            ('x', _) => break,
            _ => {}
        }
        instruction_pointer += 1;
    }
}

fn main() {
    let file = std::env::args().last().unwrap();
    let source = fs::read_to_string(file).unwrap();
    let program = parse(&source);
    interpret(program)
}
