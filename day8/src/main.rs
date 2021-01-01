use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day8_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    let mut instructions: Vec<Instruction> = Vec::with_capacity( contents.matches("\n").count() + 2 );
    //instructions.push(("nop",0,false));

    for line in contents.split('\n') {
        // assume input is nice with our parser for now
        let l: Vec<&str> = line.split(' ').collect();
        let cmd = l[0];
        let num: i32 = l[1].parse().unwrap();
        //println!("cmd: {} / num: {}", cmd, num);
        instructions.push(Instruction{ cmd,num, seen: false });
    }
    let instructions_orig = instructions.clone();

    // part 1
    let mut state = State{ jmp: 0, acc: 0 };
    loop {
        let instruction = &mut instructions.get_mut(state.jmp as usize).unwrap();
        //println!("Instruction: {:?} / State: {:?}", instruction, state);
        if instruction.seen {
            break;
        }
        instruction.seen = true;
        state = next_step(instruction, state);
    }
    println!("Accumulator pt. 1: {}", state.acc);

    // part 2
    for (idx, ins) in instructions_orig.iter().enumerate() {
        let mut test_code = instructions_orig.clone();
        let test_instruction = &mut test_code.get_mut(idx).unwrap();
        if ins.cmd == "nop" {
            test_instruction.cmd = "jmp";
        } else if ins.cmd == "jmp" {
            test_instruction.cmd = "nop";
        }
        //println!("Modifying {:?} at {}", test_instruction,idx);
        let (res, state) = finishes(&mut test_code);
        if res {
            println!("Accumulator pt. 2: {}", state.acc);
        }
    }
    Ok(())
}

fn next_step(instruction: &Instruction, orig_state: State) -> State {
    let mut state = orig_state;

    state.jmp += match instruction.cmd {
        "acc" => {
            //println!("  adding {}", instruction.num);
            state.acc += instruction.num;
            1
        },
        "jmp" => {
            //println!("  jumping {}", instruction.num);
            instruction.num
        },
        _ => {
            //println!("  something else");
            1
        }
    };
    state
}

fn finishes(instructions: &mut Vec<Instruction>) -> (bool, State) {
    let mut state = State{ jmp: 0, acc: 0 };
    loop {
        if state.jmp > (instructions.len() - 1) as i32 {
            break;
        }
        let instruction = instructions.get_mut(state.jmp as usize).unwrap();
        //println!("Instruction: {:?} / State: {:?}", instruction, state);
        if instruction.seen {
            return (false, state)
        }
        instruction.seen = true;
        state = next_step(instruction, state);
    };
    (true, state)
}

#[derive(Clone, Debug)]
struct Instruction<'a> {
    cmd: &'a str,
    num: i32,
    seen: bool
}

#[derive(Debug)]
struct State {
    jmp: i32,
    acc: i32,
}
