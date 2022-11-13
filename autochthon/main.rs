use std::{ops::Index, io::Read};



fn convert(a: u8, b: u8, c: u8, d: u8) -> i32 {
    (a as i32) << 24 |
    (b as i32) << 16 |
    (c as i32) << 8  |
    (d as i32) << 0
}

#[repr(u8)]
enum Register {
    Ip = 0, Sp, Bp
}
const NUM_REGISTERS: usize = Register::Bp as usize;

struct RegisterState {
    regs: [i32; NUM_REGISTERS]
}

impl Index<Register> for RegisterState {
    type Output = i32;

    fn index(&self, index: Register) -> &Self::Output {
        &self.regs[index as usize]
    }
}

#[derive(Debug)]
enum Instruction {
	Call(i32), /* 0 */
	Ret(), /* 1 */
	WriteByte(u8), /* 2 */
	ReadByte(u8), /* 3 */
	Debug(), /* 4 */
	Exit(), /* 5 */
	Load(i32, u8), /* 6 */
	Store(u8, i32), /* 7 */
	Fill(i32, u8), /* 8 */
	Push(u8), /* 9 */
	Pop(u8), /* 10 */
	Nop(), /* 11 */
	Add(u8, u8, u8), /* 12 */
	Subtracy(u8, u8, u8), /* 13 */
	Multiply(u8, u8, u8), /* 14 */
	Divide(u8, u8, u8), /* 15 */
}

impl Instruction {
	fn size(&self) -> i8 {
		match self {
			Self::Call(_) => 5,
			Self::Ret() => 1,
			Self::WriteByte(_) => 2,
			Self::ReadByte(_) => 2,
			Self::Debug() => 1,
			Self::Exit() => 1,
			Self::Load(_, _) => 6,
			Self::Store(_, _) => 6,
			Self::Push(_) => 2,
			Self::Pop(_) => 2,
			Self::Fill(_, _) => 6,
			Self::Nop() => 1,
			Self::Add(_, _, _) => 4,
			Self::Subtracy(_, _, _) => 4,
			Self::Multiply(_, _, _) => 4,
			Self::Divide(_, _, _) => 4,
		}
	}
    fn decode(bytes: &[u8]) -> Option<Self> {
        match bytes {
            [0, b0, b1, b2, b3,  ..] => Some(Self::Call(convert(*b0, *b1, *b2, *b3))),
            [1,  ..] => Some(Self::Ret()),
            [2, b0,  ..] => Some(Self::WriteByte(*b0)),
            [3, b0,  ..] => Some(Self::ReadByte(*b0)),
            [4,  ..] => Some(Self::Debug()),
            [5,  ..] => Some(Self::Exit()),
            [6, b0, b1, b2, b3, b4,  ..] => Some(Self::Load(convert(*b0, *b1, *b2, *b3), *b4)),
            [7, b0, b1, b2, b3, b4,  ..] => Some(Self::Store(*b0, convert(*b1, *b2, *b3, *b4))),
            [8, b0,  ..] => Some(Self::Push(*b0)),
            [9, b0,  ..] => Some(Self::Pop(*b0)),
            [10, b0, b1, b2, b3, b4,  ..] => Some(Self::Fill(convert(*b0, *b1, *b2, *b3), *b4)),
            [11,  ..] => Some(Self::Nop()),
            [12, b0, b1, b2,  ..] => Some(Self::Add(*b0, *b1, *b2)),
            [13, b0, b1, b2,  ..] => Some(Self::Subtracy(*b0, *b1, *b2)),
            [14, b0, b1, b2,  ..] => Some(Self::Multiply(*b0, *b1, *b2)),
            [15, b0, b1, b2,  ..] => Some(Self::Divide(*b0, *b1, *b2)),
            _ => None
        }
    }

    fn registers_used(&self) -> Option<Vec<u8>> {
        match self {
            // Self::Multiply(a, b, c) => Some([*a, *b, *c]),
            _ => None
        }
    }
}


struct Memory {
    instructions: Vec<u8>,
    data: Vec<u8>,
}

trait IO {
    fn read_byte(&self) -> u8;
    fn write_byte(&self, _: u8);
}


/*
    Should the interp have access to symbol table? Or is that just a link time
    thing. To simplify, I think it should just be link time.
*/
struct Interpreter {
    memory: Memory,
    registers: RegisterState,
    io: dyn IO
}

enum ExecutionError {
    InvalidInstruction(&'static str),
    Overflow,
    DivideByZero,
    UnableToFetch
}
enum ExecutionEffect {
    /// Normal
    StateChange,
    /// Something fatal happened
    Error(ExecutionError),
    /// Clean exit
    Exit
}

/*
    What is the error handling story for interpreter?

*/
impl Interpreter {

    fn step(&mut self) -> ExecutionEffect {
        match self.fetch_instruction() {
            Err(error) => return ExecutionEffect::Error(error),
            Ok(instr) => self.execute_instruction(instr)
        }
    }

    fn fetch_instruction(&self) -> Result<Instruction, ExecutionError> {
        let ip = self.registers[Register::Ip];

        if ip < 0 || ip as usize > self.memory.instructions.len() {
            return Err(ExecutionError::UnableToFetch)
        }
        let memory_slice = &self.memory.instructions[ip as usize ..];

        Instruction::decode(memory_slice)
            .and_then(Self::validate_instruction)
            .ok_or(ExecutionError::InvalidInstruction("invalid opcode"))
    }

    fn validate_instruction(i: Instruction) -> Option<Instruction> {
        Some(i)
    }

    fn execute_instruction(&self, instr: Instruction) -> ExecutionEffect {
        match instr {
            Instruction::Add(r1, r2, r3) => {
                ExecutionEffect::StateChange
            },
            i => panic!("not implemented: {:?}", i)
        }
    }

    /*
            let ip = self.registers[Register::Ip];
        if let Some(next_instruction) = Instruction::decode(&self.memory.instructions[ip as usize ..]) {
            println!("Found instruction: {:?} of size {:?}", next_instruction, next_instruction.size());
            self.regs[Register::Ip] += next_instruction.size() as i32;
            for reg in next_instruction.registers_used() {
                println!("reg = {:?}", reg);
            }
        } else {
            eprintln!("Invalid instruction");
        } */
}


struct UnixIO {
    
}

struct TestIO {

}

impl IO for UnixIO {
    fn read_byte(&self) -> u8 {
        let mut buf = [0u8];
        std::io::stdin().read_exact(&mut buf).expect("asdf");
        buf[0]
    }
    fn write_byte(&self, _: u8) {
        
    }
}

fn main() {

    //println!("Hello, world! {:?}", Instruction::decode(&[7, 0x0, 0x0, 0x0, 0xc, 0xff]));
}
