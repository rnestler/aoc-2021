use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts.next().ok_or("First part missing")?;
        let arg = parts.next().ok_or("Second part missing")?.parse().map_err(|_| "Failed to parse int")?;
        match command {
            "forward" => Ok(Command::Forward(arg)),
            "down" => Ok(Command::Down(arg)),
            "up" => Ok(Command::Up(arg)),
            _ => Err(format!("Unknown command {}", command)),
        }
    }
}

struct State {
    depth: u32,
    horizontal_position: u32,
}

impl State {
    fn new() -> State {
        State {
            depth: 0,
            horizontal_position: 0,
        }
    }
    fn run(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Forward(arg) => self.horizontal_position += arg,
                Command::Down(arg) => self.depth += arg,
                Command::Up(arg) => self.depth -= arg,
            }
        }
    }
}

fn parse(input: &str) -> Vec<Command> { 
    input.lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let input = parse(&contents);
    let mut state = State::new();
    state.run(&input);

    let solution_1 = state.depth * state.horizontal_position;
    println!("Part 1: {}", solution_1);
    

    Ok(())
}
