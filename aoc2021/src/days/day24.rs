pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let (i1, i14) = find_max(|i| (i, i + 6));
	let (i2, i13) = find_max(|i| (i, i - 2));
	let (i3, i12) = find_max(|i| (i, i + 5));
	let (i4, i5) = find_max(|i| (i, i - 5));
	let (i6, i11) = find_max(|i| (i, i + 8));
	let (i7, i8) = find_max(|i| (i, i - 4));
	let (i9, i10) = find_max(|i| (i, i + 2));

	let n = format!(
		"{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
		i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14
	)
	.parse()?;

	let mut alu = Alu::new(input);
	alu.process(n);

	if alu.z == 0 {
		Ok(n)
	} else {
		Err(anyhow::anyhow!("invalid solution computed"))
	}
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let (i1, i14) = find_min(|i| (i, i + 6));
	let (i2, i13) = find_min(|i| (i, i - 2));
	let (i3, i12) = find_min(|i| (i, i + 5));
	let (i4, i5) = find_min(|i| (i, i - 5));
	let (i6, i11) = find_min(|i| (i, i + 8));
	let (i7, i8) = find_min(|i| (i, i - 4));
	let (i9, i10) = find_min(|i| (i, i + 2));

	let n = format!(
		"{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
		i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14
	)
	.parse()?;

	let mut alu = Alu::new(input);
	alu.process(n);

	if alu.z == 0 {
		Ok(n)
	} else {
		Err(anyhow::anyhow!("invalid solution computed"))
	}
}

fn find_min<F: FnMut(isize) -> (isize, isize)>(f: F) -> (isize, isize) {
	(1..=9)
		.map(f)
		.find(|&(a, b)| a > 0 && b > 0 && a <= 9 && b <= 9)
		.unwrap()
}

fn find_max<F: FnMut(isize) -> (isize, isize)>(f: F) -> (isize, isize) {
	(1..=9)
		.rev()
		.map(f)
		.find(|&(a, b)| a > 0 && b > 0 && a <= 9 && b <= 9)
		.unwrap()
}

struct Alu<'a> {
	input: &'a str,
	w: isize,
	x: isize,
	y: isize,
	z: isize,
}

impl<'a> Alu<'a> {
	fn new(input: &'a str) -> Self {
		Self {
			input,
			w: 0,
			x: 0,
			y: 0,
			z: 0,
		}
	}

	fn process(&mut self, number: usize) {
		let number = format!("{}", number);
		let mut number = number.chars();

		self.input.lines().for_each(|line| {
			if let Some((command, args)) = line.split_once(' ') {
				match command {
					"inp" => match args {
						"w" => self.w = number.next().unwrap().to_digit(10).unwrap() as isize,
						"x" => self.x = number.next().unwrap().to_digit(10).unwrap() as isize,
						"y" => self.y = number.next().unwrap().to_digit(10).unwrap() as isize,
						"z" => self.z = number.next().unwrap().to_digit(10).unwrap() as isize,
						_ => unreachable!(),
					},

					"add" => {
						let (a, b) = args.split_once(' ').unwrap();
						match (a, b) {
							("w", "x") => self.w += self.x,
							("w", "y") => self.w += self.y,
							("w", "z") => self.w += self.z,
							("w", _) => self.w += b.parse::<isize>().unwrap(),

							("x", "w") => self.x += self.w,
							("x", "y") => self.x += self.y,
							("x", "z") => self.x += self.z,
							("x", _) => self.x += b.parse::<isize>().unwrap(),

							("y", "w") => self.y += self.w,
							("y", "x") => self.y += self.x,
							("y", "z") => self.y += self.z,
							("y", _) => self.y += b.parse::<isize>().unwrap(),

							("z", "w") => self.z += self.w,
							("z", "x") => self.z += self.x,
							("z", "y") => self.z += self.y,
							("z", _) => self.z += b.parse::<isize>().unwrap(),

							_ => unreachable!(),
						}
					}

					"mul" => {
						let (a, b) = args.split_once(' ').unwrap();
						match (a, b) {
							("w", "x") => self.w *= self.x,
							("w", "y") => self.w *= self.y,
							("w", "z") => self.w *= self.z,
							("w", _) => self.w *= b.parse::<isize>().unwrap(),

							("x", "w") => self.x *= self.w,
							("x", "y") => self.x *= self.y,
							("x", "z") => self.x *= self.z,
							("x", _) => self.x *= b.parse::<isize>().unwrap(),

							("y", "w") => self.y *= self.w,
							("y", "x") => self.y *= self.x,
							("y", "z") => self.y *= self.z,
							("y", _) => self.y *= b.parse::<isize>().unwrap(),

							("z", "w") => self.z *= self.w,
							("z", "x") => self.z *= self.x,
							("z", "y") => self.z *= self.y,
							("z", _) => self.z *= b.parse::<isize>().unwrap(),

							_ => unreachable!(),
						}
					}

					"div" => {
						let (a, b) = args.split_once(' ').unwrap();
						match (a, b) {
							("w", "x") => self.w /= self.x,
							("w", "y") => self.w /= self.y,
							("w", "z") => self.w /= self.z,
							("w", _) => self.w /= b.parse::<isize>().unwrap(),

							("x", "w") => self.x /= self.w,
							("x", "y") => self.x /= self.y,
							("x", "z") => self.x /= self.z,
							("x", _) => self.x /= b.parse::<isize>().unwrap(),

							("y", "w") => self.y /= self.w,
							("y", "x") => self.y /= self.x,
							("y", "z") => self.y /= self.z,
							("y", _) => self.y /= b.parse::<isize>().unwrap(),

							("z", "w") => self.z /= self.w,
							("z", "x") => self.z /= self.x,
							("z", "y") => self.z /= self.y,
							("z", _) => self.z /= b.parse::<isize>().unwrap(),

							_ => unreachable!(),
						}
					}

					"mod" => {
						let (a, b) = args.split_once(' ').unwrap();
						match (a, b) {
							("w", "x") => self.w %= self.x,
							("w", "y") => self.w %= self.y,
							("w", "z") => self.w %= self.z,
							("w", _) => self.w %= b.parse::<isize>().unwrap(),

							("x", "w") => self.x %= self.w,
							("x", "y") => self.x %= self.y,
							("x", "z") => self.x %= self.z,
							("x", _) => self.x %= b.parse::<isize>().unwrap(),

							("y", "w") => self.y %= self.w,
							("y", "x") => self.y %= self.x,
							("y", "z") => self.y %= self.z,
							("y", _) => self.y %= b.parse::<isize>().unwrap(),

							("z", "w") => self.z %= self.w,
							("z", "x") => self.z %= self.x,
							("z", "y") => self.z %= self.y,
							("z", _) => self.z %= b.parse::<isize>().unwrap(),

							_ => unreachable!(),
						}
					}

					"eql" => {
						let (a, b) = args.split_once(' ').unwrap();
						match (a, b) {
							("w", "x") => self.w = isize::from(self.w == self.x),
							("w", "y") => self.w = isize::from(self.w == self.y),
							("w", "z") => self.w = isize::from(self.w == self.z),
							("w", _) => self.w = isize::from(self.w == b.parse::<isize>().unwrap()),

							("x", "w") => self.x = isize::from(self.x == self.w),
							("x", "y") => self.x = isize::from(self.x == self.y),
							("x", "z") => self.x = isize::from(self.x == self.z),
							("x", _) => self.x = isize::from(self.x == b.parse::<isize>().unwrap()),

							("y", "w") => self.y = isize::from(self.y == self.w),
							("y", "x") => self.y = isize::from(self.y == self.x),
							("y", "z") => self.y = isize::from(self.y == self.z),
							("y", _) => self.y = isize::from(self.y == b.parse::<isize>().unwrap()),

							("z", "w") => self.z = isize::from(self.z == self.w),
							("z", "x") => self.z = isize::from(self.z == self.x),
							("z", "y") => self.z = isize::from(self.z == self.y),
							("z", _) => self.z = isize::from(self.z == b.parse::<isize>().unwrap()),

							_ => unreachable!(),
						}
					}

					_ => unimplemented!("{}", command),
				}
			} else {
				panic!("Invalid line: {}", line);
			}
		})
	}
}

advent::problem!(
	r#"
		inp w
		mul x 0
		add x z
		mod x 26
		div z 1
		add x 13
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 6
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 1
		add x 15
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 7
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 1
		add x 15
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 10
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 1
		add x 11
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 2
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 26
		add x -7
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 15
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 1
		add x 10
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 8
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 1
		add x 10
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 1
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 26
		add x -5
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 10
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 1
		add x 15
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 5
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 26
		add x -3
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 3
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 26
		add x 0
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 5
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 26
		add x -5
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 11
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 26
		add x -9
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 12
		mul y x
		add z y
		inp w
		mul x 0
		add x z
		mod x 26
		div z 26
		add x 0
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y 10
		mul y x
		add z y
	"#,
	39494195799979,
	13161151139617,
);
