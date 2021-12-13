use std::collections::HashMap;

fn pt1(input: &str) -> anyhow::Result<usize> {
	dbg!(input);

	let (points, folds) = input.split_once("\n\n").unwrap();

	let mut grid: HashMap<(isize, isize), _> = HashMap::new();
	points.lines().for_each(|line| {
		let (x, y) = line.split_once(',').unwrap();
		let x = x.parse::<isize>().unwrap();
		let y = y.parse::<isize>().unwrap();

		grid.insert((x, y), Point::Dot);
	});

	let mut result = folds.lines().map(|line| {
		let (_, amount) = line.split_once("=").unwrap();
		let amount = amount.parse::<isize>().unwrap();

		if line.contains("x=") {
			grid.clone()
				.keys()
				.filter(|(x, _)| *x > amount)
				.for_each(|(x, y)| match grid.remove(&(*x, *y)) {
					Some(Point::Dot) => {
						dbg!(((x, y), (amount - (x - amount).abs(), y)));

						grid.insert((amount - (x - amount).abs(), *y), Point::Dot);
					}
					_ => {}
				});
		} else {
			grid.clone()
				.keys()
				.filter(|(_, y)| *y > amount)
				.for_each(|(x, y)| match grid.remove(&(*x, *y)) {
					Some(Point::Dot) => {
						dbg!(((x, y), (*x, amount - (y - amount).abs())));

						grid.insert((*x, amount - (y - amount).abs()), Point::Dot);
					}
					_ => {}
				});
		}

		grid.iter()
			.map(|(coords, point)| {
				dbg!(coords);

				(coords, point)
			})
			.filter(|(_, point)| matches!(point, Point::Dot))
			.count()
	});

	Ok(result.next().unwrap())
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	let (points, folds) = input.split_once("\n\n").unwrap();

	let mut grid: HashMap<(isize, isize), _> = HashMap::new();
	points.lines().for_each(|line| {
		let (x, y) = line.split_once(',').unwrap();
		let x = x.parse::<isize>().unwrap();
		let y = y.parse::<isize>().unwrap();

		grid.insert((x, y), Point::Dot);
	});

	folds.lines().for_each(|line| {
		let (_, amount) = line.split_once("=").unwrap();
		let amount = amount.parse::<isize>().unwrap();

		if line.contains("x=") {
			grid.clone()
				.keys()
				.filter(|(x, _)| *x > amount)
				.for_each(|(x, y)| match grid.remove(&(*x, *y)) {
					Some(Point::Dot) => {
						dbg!(((x, y), (amount - (x - amount).abs(), y)));

						grid.insert((amount - (x - amount).abs(), *y), Point::Dot);
					}
					_ => {}
				});
		} else {
			grid.clone()
				.keys()
				.filter(|(_, y)| *y > amount)
				.for_each(|(x, y)| match grid.remove(&(*x, *y)) {
					Some(Point::Dot) => {
						dbg!(((x, y), (*x, amount - (y - amount).abs())));

						grid.insert((*x, amount - (y - amount).abs()), Point::Dot);
					}
					_ => {}
				});
		}
	});

	let (max_x, _) = grid.keys().max_by_key(|(x, _)| *x).unwrap();
	let (_, max_y) = grid.keys().max_by_key(|(_, y)| *y).unwrap();

	for y in 0..=*max_y {
		for x in 0..=*max_x {
			if grid.contains_key(&(x, y)) {
				print!("#");
			} else {
				print!(" ");
			}
		}

		println!();
	}

	Ok(1)
}

#[derive(Debug, Clone, Copy)]
enum Point {
	Empty,
	Dot,
}

advent::problem!(
	r#"
		6,10
		0,14
		9,10
		0,3
		10,4
		4,11
		6,0
		6,12
		4,1
		0,13
		10,12
		3,4
		3,0
		8,4
		1,10
		2,14
		8,10
		9,0

		fold along y=7
		fold along x=5
	"#,
	17,
	0,
);
