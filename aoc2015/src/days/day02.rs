pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			let (l, rest) = line.split_once('x').unwrap();
			let (w, h) = rest.split_once('x').unwrap();

			let (l, w, h): (isize, isize, isize) =
				(l.parse().unwrap(), w.parse().unwrap(), h.parse().unwrap());

			let lw = w * l;
			let wh = h * w;
			let lh = l * h;

			(2 * l * w) + (2 * w * h) + (2 * h * l) + lw.min(wh).min(lh)
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			let (l, rest) = line.split_once('x').unwrap();
			let (w, h) = rest.split_once('x').unwrap();

			let (l, w, h): (isize, isize, isize) =
				(l.parse().unwrap(), w.parse().unwrap(), h.parse().unwrap());

			(l * 2) + (w * 2) + (h * 2) - (l.max(w).max(h) * 2) + (l * w * h)
		})
		.sum())
}

advent::problem!(
	r#"
		2x3x4
		1x1x10
    "#,
	101,
	48,
);
