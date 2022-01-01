pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let (mut x, mut y): (i8, i8) = (0, 0);
	let mut code = 0;

	input.lines().for_each(|line| {
		line.chars().for_each(|c| match c {
			'R' => x = (x + 1).min(1),
			'L' => x = (x - 1).max(-1),
			'D' => y = (y + 1).min(1),
			'U' => y = (y - 1).max(-1),
			_ => unreachable!(),
		});

		code = code * 10
			+ match (x, y) {
				(-1, -1) => 1,
				(0, -1) => 2,
				(1, -1) => 3,
				(-1, 0) => 4,
				(0, 0) => 5,
				(1, 0) => 6,
				(-1, 1) => 7,
				(0, 1) => 8,
				(1, 1) => 9,
				_ => unreachable!(),
			};
	});

	Ok(code)
}

pub fn pt2(input: &str) -> anyhow::Result<String> {
	let (mut x, mut y): (i8, i8) = (-2, 0);
	let mut code = String::with_capacity(input.lines().count());

	input.lines().for_each(|line| {
		line.chars().for_each(|c| {
			match (c, x, y) {
				('R', _, -2) => {}
				('R', 1, -1) => {}
				('R', 2, 0) => {}
				('R', 1, 1) => {}
				('R', _, 2) => {}
				('L', _, -2) => {}
				('L', -1, -1) => {}
				('L', -2, 0) => {}
				('L', -1, 1) => {}
				('L', _, 2) => {}
				('U', -2, _) => {}
				('U', -1, -1) => {}
				('U', 0, -2) => {}
				('U', 1, -1) => {}
				('U', 2, _) => {}
				('D', -2, _) => {}
				('D', -1, 1) => {}
				('D', 0, 2) => {}
				('D', 1, 1) => {}
				('D', 2, _) => {}
				('R', _, _) => x = (x + 1).min(2),
				('L', _, _) => x = (x - 1).max(-2),
				('U', _, _) => y = (y - 1).max(-2),
				('D', _, _) => y = (y + 1).min(2),
				_ => unreachable!(),
			};
		});

		code.push(match (x, y) {
			(0, -2) => '1',
			(-1, -1) => '2',
			(0, -1) => '3',
			(1, -1) => '4',
			(-2, 0) => '5',
			(-1, 0) => '6',
			(0, 0) => '7',
			(1, 0) => '8',
			(2, 0) => '9',
			(-1, 1) => 'A',
			(0, 1) => 'B',
			(1, 1) => 'C',
			(0, 2) => 'D',
			_ => unreachable!(),
		});
	});

	Ok(code)
}

advent::problem!(
	r#"
		ULL
		RRDDD
		LURDL
		UUUUD
    "#,
	1985,
	"5DB3",
);
