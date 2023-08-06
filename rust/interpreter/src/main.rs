use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{stdout, Write};

#[derive(Debug)]
struct Program {
    tokens: Vec<char>,
    opening_to_closing: HashMap<usize, usize>,
    closing_to_opening: HashMap<usize, usize>,
}

fn parse(program: &str) -> Program {
    let mut tokens: Vec<char> = vec![];
    let mut opened_brackets: Vec<usize> = vec![];
    let mut matching_brackets: Vec<(usize, usize)> = vec![];
    let mut opening_to_closing: HashMap<usize, usize> = HashMap::new();
    let mut closing_to_opening: HashMap<usize, usize> = HashMap::new();

    let valit_tokens: HashSet<char> = HashSet::from(['<', '>', '+', '-', '[', ']', '.']);

    for (i, token) in program
        .chars()
        .filter(|ch| valit_tokens.contains(ch))
        .enumerate()
    {
        match token {
            '<' | '>' | '+' | '-' | '.' => tokens.push(token),
            '[' => {
                opened_brackets.push(i);
                tokens.push(token);
            }
            ']' => {
                let opening = opened_brackets.pop().unwrap();
                matching_brackets.push((opening, i));
                tokens.push(token);
            }
            _ => {}
        }
    }
    tokens.push('x');

    for (opening, closing) in matching_brackets.iter() {
        opening_to_closing.insert(*opening, *closing);
        closing_to_opening.insert(*closing, *opening);
    }

    Program {
        tokens,
        opening_to_closing,
        closing_to_opening,
    }
}

fn interpret(program: Program) {
    let mut memory: Vec<u8> = vec![0; 65536];
    let mut instruction_pointer: usize = 0;
    let mut data_pointer: usize = 0;

    loop {
        let token = program.tokens[instruction_pointer];
        match token {
            '+' => memory[data_pointer] += 1,
            '-' => memory[data_pointer] -= 1,
            '>' => data_pointer += 1,
            '<' => data_pointer -= 1,
            '[' => {
                if memory[data_pointer] == 0 {
                    instruction_pointer = *program
                        .opening_to_closing
                        .get(&instruction_pointer)
                        .unwrap();
                }
            }
            ']' => {
                if memory[data_pointer] != 0 {
                    instruction_pointer = *program
                        .closing_to_opening
                        .get(&instruction_pointer)
                        .unwrap();
                }
            }
            '.' => {
                print!("{}", memory[data_pointer] as char);
                let _ = stdout().flush();
            }
            'x' => break,
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
