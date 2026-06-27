use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::{self, Read, Write},
};
const TAPE_LENGTH: usize = 30000;

fn interpret(instructions: &str) -> Result<(), String> {
    let instructions = instructions.bytes().collect::<Vec<u8>>();
    let ins_length = instructions.len();

    let mut tape: Vec<u8> = vec![0u8; TAPE_LENGTH];

    let mut tape_index: usize = 0;
    let mut ins_index = 0;

    let mut jump_table: HashMap<usize, usize> = HashMap::new();

    let mut stack: Vec<usize> = Vec::new();

    for (i, char) in instructions.iter().enumerate() {
        if *char == b'[' {
            stack.push(i);
        } else if *char == b']' {
            if stack.is_empty() {
                return Err("String is unbalanced".into());
            } else {
                if let Some(b) = stack.pop() {
                    // if brackets match
                    if instructions[b] == b'[' {
                        // add entries to jump table
                        jump_table.entry(i).or_insert(0);
                        jump_table.entry(i).insert_entry(b);
                        jump_table.entry(b).or_insert(0);
                        jump_table.entry(b).insert_entry(i);
                    }
                }
            }
        }
    }

    if !stack.is_empty() {
        return Err("Unbalanced parentheses".into());
    }

    let mut total_instructions: u128 = 0;

    while ins_index < ins_length {
        let command = instructions[ins_index];
        match command {
            b'>' => {
                if tape_index == TAPE_LENGTH - 1 {
                    // move pointer up by one index
                    return Err("Tried to move index out of bounds".into());
                }
                tape_index += 1;
            }
            b'<' => {
                // move pointer down by one index
                if tape_index == 0 {
                    return Err("Tried to move index out of bounds".into());
                }
                tape_index -= 1;
            }
            b'+' => {
                if tape[tape_index] == 255u8 {
                    // increasing by 1 % 256
                    tape[tape_index] = 0u8;
                } else {
                    tape[tape_index] += 1u8;
                }
            }
            b'-' => {
                if tape[tape_index] == 0u8 {
                    // decreasing by 1 % 256
                    tape[tape_index] = 255u8;
                } else {
                    tape[tape_index] -= 1u8;
                }
            }
            b'.' => {
                // print current value in tape cell
                // print!("{}", tape[tape_index] as char);

                io::stdout()
                    .write_all(&[tape[tape_index]])
                    .map_err(|e| "Failed to write to stdout: {e}".to_string())?;
            }
            b',' => {
                // read input

                let mut input = [0u8; 1];

                let mut input = [0u8; 1];
                io::stdout()
                    .flush()
                    .map_err(|e| format!("Failed to flush stdout: {e}"))?;
                match io::stdin()
                    .read(&mut input)
                    .map_err(|e| format!("Failed to read stdin: {e}"))?
                {
                    0 => tape[tape_index] = 0, // EOF → set cell to 0
                    _ => tape[tape_index] = input[0],
                }
            }
            b'[' => {
                // jump from left to right brackets, vice versa
                if tape[tape_index] == 0u8 {
                    ins_index = *jump_table.get(&ins_index).unwrap();
                }
            }
            b']' => {
                if tape[tape_index] != 0u8 {
                    ins_index = *jump_table.get(&ins_index).unwrap();
                }
            }
            _ => (),
        }

        ins_index += 1;
        total_instructions += 1;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: bf.exe <arg1>");
        return Err("Not enough arguments".into());
    }

    let mut contents = fs::read_to_string(&args[1])?;

    // ignore whitespace
    contents.retain(|c| !c.is_whitespace());

    let mut instructions = String::new();

    for line in contents.lines() {
        instructions.push_str(line); // push all code onto one line
    }

    interpret(instructions.as_str())?;

    Ok(())
}
