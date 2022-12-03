pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.as_bytes()
		.chunks(4)
		.map(|c| (isize::from(c[0] - b'A'), isize::from(c[2] - b'X')))
		.map(|(a, b)| b + 1 + ((b - a + 1) % 3) * 3)
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.as_bytes()
		.chunks(4)
		.map(|c| (isize::from(c[0] - b'A'), isize::from(c[2] - b'X')))
		.map(|(a, b)| match b {
			0 => (a - 1) % 3 + 1,
			1 => (a + 1) + 3,
			2 => ((a + 1) % 3 + 1) + 6,
			_ => unreachable!("invalid input character"),
		})
		.sum())
}

advent::problem!(
	r#"
		A Y
		B X
		C Z
    "#,
	15,
	12,
);
