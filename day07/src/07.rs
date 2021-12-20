pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let mut nums: Vec<isize> = input.split(',').map(|c| c.parse().unwrap()).collect();
	nums.sort_unstable();

	let mid = nums.len() / 2;
	let mut minimum = isize::MAX;

	for i in mid - 1..=mid + 1 {
		minimum = minimum.min(nums.iter().fold(0, |total, n| total + (n - nums[i]).abs()));
	}

	Ok(minimum)
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	let nums: Vec<isize> = input.split(',').map(|c| c.parse().unwrap()).collect();
	let average = (nums.iter().sum::<isize>() as f64) / (nums.len() as f64);
	let (floor, ceil) = (average.floor() as isize, average.ceil() as isize);

	let (floor, ceil) = nums.iter().fold((0, 0), |(floor_total, ceil_total), n| {
		(
			floor_total + summation((n - floor).abs()),
			ceil_total + summation((n - ceil).abs()),
		)
	});

	Ok(floor.min(ceil))
}

fn summation(n: isize) -> isize {
	n * (n + 1) / 2
}

advent::problem!(
	r#"
		16,1,2,0,4,2,7,1,2,14
	"#,
	37,
	168,
);
