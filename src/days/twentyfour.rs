#![allow(dead_code)]

use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Arithmetic Logic Unit";

#[derive(Debug)]
enum Register { W, X, Y, Z }

#[derive(Debug)]
enum Value {
    Register(Register),
    Constant(isize)
}

#[derive(Debug)]
enum Instruction {
    Input(Register),
    Add(Register, Value),
    Multiply(Register, Value),
    Divide(Register, Value),
    Mod(Register, Value),
    Equal(Register, Value)
}

#[derive(Debug)]
struct ArithmeticLogicUnit {
    instructions: Vec<Instruction>,
    input_pointer: usize,
    registers: [isize; 4],
    inputs: Vec<isize>
}

impl FromStr for Register {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => Err(AocError::Misc(format!("Bad register name \"{}\"", s)))
        }
    }
}

impl FromStr for Value {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse() {
            Ok(Value::Constant(value))
        } else {
            let register: Register = s.parse()?;
            Ok(Value::Register(register))
        }
    }
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        if tokens.len() < 2 {
            Err(AocError::Misc("Not enough tokens".to_string()))
        } else {
            match tokens[0] {
                "inp" => Ok(Instruction::Input(tokens[1].parse()?)),
                "add" => Ok(Instruction::Add(tokens[1].parse()?, tokens[2].parse()?)),
                "mul" => Ok(Instruction::Multiply(tokens[1].parse()?, tokens[2].parse()?)),
                "div" => Ok(Instruction::Divide(tokens[1].parse()?, tokens[2].parse()?)),
                "mod" => Ok(Instruction::Mod(tokens[1].parse()?, tokens[2].parse()?)),
                "eql" => Ok(Instruction::Equal(tokens[1].parse()?, tokens[2].parse()?)),
                _ => Err(AocError::Misc(format!("Bad opcode \"{}\"", tokens[0])))
            }
        }
    }
}

impl ArithmeticLogicUnit {
    fn new(program: &str, input: Vec<isize>) -> Result<Self, AocError> {
        let instructions = program.lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Instruction>, AocError>>()?;
        Ok(ArithmeticLogicUnit {
            instructions,
            input_pointer: 0,
            registers: [0, 0, 0, 0],
            inputs: input
        })
    }
}

impl Register {
    fn index(&self) -> usize {
        match self {
            Register::W => 0,
            Register::X => 1,
            Register::Y => 2,
            Register::Z => 3
        }
    }
}

impl ArithmeticLogicUnit {
    fn reset(&mut self, input: Vec<isize>) {
        self.registers = [0, 0, 0, 0];
        self.input_pointer = 0;
        self.inputs = input;
    }

    fn value(&self, value: &Value) -> isize {
        match value {
            Value::Constant(value) => *value,
            Value::Register(register) => self.registers[register.index()]
        }
    }

    fn run(&mut self) -> Result<(), AocError> {
        for instruction in &self.instructions {
            match instruction {
                Instruction::Input(reg) => {
                    if self.input_pointer >= self.inputs.len() {
                        return Err(AocError::Misc("Nnt enough inputs".to_string()));
                    }

                    self.registers[reg.index()] = self.inputs[self.input_pointer];
                    self.input_pointer += 1;
                },
                Instruction::Add(reg, val) => {
                    self.registers[reg.index()] = self.registers[reg.index()] + self.value(val)
                },
                Instruction::Multiply(reg, val) => {
                    self.registers[reg.index()] = self.registers[reg.index()] * self.value(val)
                },
                Instruction::Divide(reg, val) => {
                    self.registers[reg.index()] = self.registers[reg.index()] / self.value(val)
                },
                Instruction::Mod(reg, val) => {
                    self.registers[reg.index()] = self.registers[reg.index()] % self.value(val)
                },
                Instruction::Equal(reg, val) => {
                    if self.registers[reg.index()] == self.value(val) {
                        self.registers[reg.index()] = 1;
                    } else {
                        self.registers[reg.index()] = 0;
                    }
                }
            };
        }

        Ok(())
    }
}

fn digits(x: usize) -> Vec<isize> {
    if x < 10 {
        vec![ x as isize ]
    } else {
        let mut prefix = digits(x / 10);
        prefix.push((x % 10) as isize);
        prefix
    }
}

//  first digit sets z = digit + 12
// second digit sets z = z * 26 + 8 + digit
//  third digit sets z = z * 26 + 7 + digit
// fourth digit sets z = z * 26 + 4 + digit
// fifth digit x = z % 26 - 11, if x = digit z = 0

// TODO: Write code to actually find the right candidates

pub fn part_one(input: &str) -> Result<String, AocError> {
    let candidate = 59692994994998;

    let mut alu = ArithmeticLogicUnit::new(input, digits(candidate))?;
    alu.run()?;

    if alu.registers[Register::Z.index()] == 0 {
        Ok(candidate.to_string())
    } else {
        Err(AocError::Misc("Bad candidate".to_string()))
    }
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let candidate = 16181111641521;

    let mut alu = ArithmeticLogicUnit::new(input, digits(candidate))?;
    alu.run()?;

    if alu.registers[Register::Z.index()] == 0 {
        Ok(candidate.to_string())
    } else {
        Err(AocError::Misc("Bad candidate".to_string()))
    }
}
