use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input.chars().fold(0, |total, c| {
		total
			+ match c {
				'(' => 1,
				')' => -1,
				_ => unreachable!(),
			}
	}))
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.chars()
		.zip(1..)
		.fold_while(0, |total, (c, position)| {
			let new_total = total
				+ match c {
					'(' => 1,
					')' => -1,
					_ => unreachable!(),
				};

			if new_total < 0 {
				itertools::FoldWhile::Done(position)
			} else {
				itertools::FoldWhile::Continue(new_total)
			}
		})
		.into_inner())
}

advent::problem!(
	r#"
		()())
    "#,
	-1,
	5,
);
