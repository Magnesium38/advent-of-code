use std::collections::HashSet;

fn pt1(input: &str) -> anyhow::Result<usize> {
	let (points, folds) = input.split_once("\n\n").unwrap();

	let mut grid: HashSet<(isize, isize)> = HashSet::new();
	points.lines().for_each(|line| {
		let (x, y) = line.split_once(',').unwrap();
		let x = x.parse::<isize>().unwrap();
		let y = y.parse::<isize>().unwrap();

		grid.insert((x, y));
	});

	let mut result = folds.lines().map(|line| {
		let (_, amount) = line.split_once("=").unwrap();
		let amount = amount.parse::<isize>().unwrap();

		if line.contains("x=") {
			grid.clone()
				.iter()
				.filter(|(x, _)| *x > amount)
				.for_each(|(x, y)| {
					if grid.remove(&(*x, *y)) {
						grid.insert((amount - (x - amount).abs(), *y));
					}
				});
		} else {
			grid.clone()
				.iter()
				.filter(|(_, y)| *y > amount)
				.for_each(|(x, y)| {
					if grid.remove(&(*x, *y)) {
						grid.insert((*x, amount - (y - amount).abs()));
					}
				});
		}

		grid.len()
	});

	Ok(result.next().unwrap())
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	let (points, folds) = input.split_once("\n\n").unwrap();

	let mut grid: HashSet<(isize, isize)> = HashSet::new();
	points.lines().for_each(|line| {
		let (x, y) = line.split_once(',').unwrap();
		let x = x.parse::<isize>().unwrap();
		let y = y.parse::<isize>().unwrap();

		grid.insert((x, y));
	});

	folds.lines().for_each(|line| {
		let (_, amount) = line.split_once("=").unwrap();
		let amount = amount.parse::<isize>().unwrap();

		if line.contains("x=") {
			grid.clone()
				.iter()
				.filter(|(x, _)| *x > amount)
				.for_each(|(x, y)| {
					if grid.remove(&(*x, *y)) {
						grid.insert((amount - (x - amount).abs(), *y));
					}
				});
		} else {
			grid.clone()
				.iter()
				.filter(|(_, y)| *y > amount)
				.for_each(|(x, y)| {
					if grid.remove(&(*x, *y)) {
						grid.insert((*x, amount - (y - amount).abs()));
					}
				});
		}
	});

	let (max_x, _) = grid.iter().max_by_key(|(x, _)| *x).unwrap();
	let (_, max_y) = grid.iter().max_by_key(|(_, y)| *y).unwrap();

	for y in 0..=*max_y {
		for x in 0..=*max_x {
			if grid.contains(&(x, y)) {
				print!("#");
			} else {
				print!(" ");
			}
		}

		println!();
	}

	Ok(1)
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
