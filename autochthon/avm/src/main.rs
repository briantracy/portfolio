

fn convert(a: u8, b: u8, c: u8, d: u8) -> i32 {
    (a as i32) << 24 |
    (b as i32) << 16 |
    (c as i32) << 8  |
    (d as i32) << 0
}

#[derive(Debug)]
enum Instruction {
	Call(i32),
	Ret(),
	WriteByte(u8),
	ReadByte(u8),
	Nop(),
	Debug(),
	Exit(),
	Load(i32, u8),
	Store(u8, i32),
	Add(u8, u8, u8),
	Subtracy(u8, u8, u8),
	Multiply(u8, u8, u8),
	Divide(u8, u8, u8),
}

impl Instruction {
	fn size(&self) -> i8 {
		match self {
			Self::Call(_) => 5,
			Self::Ret() => 1,
			Self::WriteByte(_) => 2,
			Self::ReadByte(_) => 2,
			Self::Nop() => 1,
			Self::Debug() => 1,
			Self::Exit() => 1,
			Self::Load(_, _) => 6,
			Self::Store(_, _) => 6,
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
            [4,  ..] => Some(Self::Nop()),
            [5,  ..] => Some(Self::Debug()),
            [6,  ..] => Some(Self::Exit()),
            [7, b0, b1, b2, b3, b4,  ..] => Some(Self::Load(convert(*b0, *b1, *b2, *b3), *b4)),
            [8, b0, b1, b2, b3, b4,  ..] => Some(Self::Store(*b0, convert(*b1, *b2, *b3, *b4))),
            [9, b0, b1, b2,  ..] => Some(Self::Add(*b0, *b1, *b2)),
            [10, b0, b1, b2,  ..] => Some(Self::Subtracy(*b0, *b1, *b2)),
            [11, b0, b1, b2,  ..] => Some(Self::Multiply(*b0, *b1, *b2)),
            [12, b0, b1, b2,  ..] => Some(Self::Divide(*b0, *b1, *b2)),
            _ => None
        }
    }
}

struct Interpreter {
    instructions: Vec<u8>,
    ip: i32
}

impl Interpreter {
    fn step(&mut self) {
        if let Some(next_instruction) = Instruction::decode(&self.instructions[self.ip as usize ..]) {
            println!("Found instruction: {:?} of size {:?}", next_instruction, next_instruction.size());
            self.ip += next_instruction.size() as i32;
        } else {
            eprintln!("Invalid instruction");
        }

    }
}


fn main() {
    let mut j = Interpreter { instructions: vec![1,2,3], ip: 0 };
    j.step();
    j.step();
    j.step();
    //println!("Hello, world! {:?}", Instruction::decode(&[7, 0x0, 0x0, 0x0, 0xc, 0xff]));
}
