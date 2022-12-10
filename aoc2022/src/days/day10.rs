use std::str::Lines;

#[derive(Copy, Clone, Debug)]
enum Instruction {
	Noop,
	Addx(isize),
}

struct InstructionSet<'a> {
	input: Lines<'a>,
	current_instruction: Option<Instruction>,
	count: usize,
}

impl<'a> Iterator for InstructionSet<'a> {
	type Item = (usize, Instruction);

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;

		if let Some(ins) = self.current_instruction {
			self.current_instruction = None;

			Some(ins)
		} else {
			self.input.next().map(|line| match line.chars().next() {
				Some('a') => {
					self.current_instruction =
						Some(Instruction::Addx(line[5..].parse::<isize>().unwrap()));
					Instruction::Noop
				}
				Some(_) => Instruction::Noop,
				_ => unreachable!(),
			})
		}
		.map(|ins| (self.count, ins))
	}
}

impl<'a> InstructionSet<'a> {
	fn new(input: &'a str) -> Self {
		InstructionSet {
			input: input.lines(),
			current_instruction: None,
			count: 0,
		}
	}
}

fn draw(sprite: &mut [bool; 40], index: isize) {
	if let Ok(i) = usize::try_from(index) {
		if (0..40).contains(&i) {
			sprite[i] = true;
		}
	}
}

fn draw_sprite(sprite: &mut [bool; 40], x: isize) {
	*sprite = [false; 40];
	draw(sprite, x - 1);
	draw(sprite, x);
	draw(sprite, x + 1);
}

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let mut total: isize = 0;
	let mut x: isize = 1;

	for (cycle, instruction) in InstructionSet::new(input) {
		if matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220) {
			total += x * isize::try_from(cycle).unwrap();
		}

		match instruction {
			Instruction::Noop => (),
			Instruction::Addx(v) => x += v,
		}
	}

	Ok(total)
}

pub fn pt2(input: &str) -> anyhow::Result<String> {
	let mut s = String::new();
	let mut sprite = [false; 40];
	let mut x = 1_isize;
	draw_sprite(&mut sprite, x);

	for (cycle, instruction) in InstructionSet::new(input) {
		draw_sprite(&mut sprite, x);

		s.push(if sprite[(cycle - 1) % 40] { '#' } else { '.' });
		if cycle % 40 == 0 {
			s.push('\n')
		}

		match instruction {
			Instruction::Noop => (),
			Instruction::Addx(v) => x += v,
		}
	}

	s.pop();

	Ok(s)
}

advent::problem!(
	r#"
		addx 15
		addx -11
		addx 6
		addx -3
		addx 5
		addx -1
		addx -8
		addx 13
		addx 4
		noop
		addx -1
		addx 5
		addx -1
		addx 5
		addx -1
		addx 5
		addx -1
		addx 5
		addx -1
		addx -35
		addx 1
		addx 24
		addx -19
		addx 1
		addx 16
		addx -11
		noop
		noop
		addx 21
		addx -15
		noop
		noop
		addx -3
		addx 9
		addx 1
		addx -3
		addx 8
		addx 1
		addx 5
		noop
		noop
		noop
		noop
		noop
		addx -36
		noop
		addx 1
		addx 7
		noop
		noop
		noop
		addx 2
		addx 6
		noop
		noop
		noop
		noop
		noop
		addx 1
		noop
		noop
		addx 7
		addx 1
		noop
		addx -13
		addx 13
		addx 7
		noop
		addx 1
		addx -33
		noop
		noop
		noop
		addx 2
		noop
		noop
		noop
		addx 8
		noop
		addx -1
		addx 2
		addx 1
		noop
		addx 17
		addx -9
		addx 1
		addx 1
		addx -3
		addx 11
		noop
		noop
		addx 1
		noop
		addx 1
		noop
		noop
		addx -13
		addx -19
		addx 1
		addx 3
		addx 26
		addx -30
		addx 12
		addx -1
		addx 3
		addx 1
		noop
		noop
		noop
		addx -9
		addx 18
		addx 1
		addx 2
		noop
		noop
		addx 9
		noop
		noop
		noop
		addx -1
		addx 2
		addx -37
		addx 1
		addx 3
		noop
		addx 15
		addx -21
		addx 22
		addx -6
		addx 1
		noop
		addx 2
		addx 1
		noop
		addx -10
		noop
		noop
		addx 20
		addx 1
		addx 2
		addx 2
		addx -6
		addx -11
		noop
		noop
		noop
    "#,
	13140,
	r#"
		##..##..##..##..##..##..##..##..##..##..
		###...###...###...###...###...###...###.
		####....####....####....####....####....
		#####.....#####.....#####.....#####.....
		######......######......######......####
		#######.......#######.......#######.....
	"#
	.trim()
	.lines()
	.map(|line| line.trim())
	.collect::<Vec<_>>()
	.join("\n"),
);
