use aoc2015::days::*;
use std::{fs::read_to_string, path::PathBuf};

fn main() -> anyhow::Result<()> {
	let day = std::env::args()
		.nth(1)
		.ok_or_else(|| anyhow::anyhow!("missing day"))?
		.parse::<isize>()?;

	if !(1..=25).contains(&day) {
		return Err(anyhow::anyhow!("invalid day; expecting 1-25"));
	}

	let input = get_input(day)?;

	match day {
		1 => run(&input, day01::pt1, day01::pt2)?,
		2 => run(&input, day02::pt1, day02::pt2)?,
		3 => run(&input, day03::pt1, day03::pt2)?,
		4 => run(&input, day04::pt1, day04::pt2)?,
		5 => run(&input, day05::pt1, day05::pt2)?,
		6 => run(&input, day06::pt1, day06::pt2)?,
		7 => run(&input, day07::pt1, day07::pt2)?,
		8 => run(&input, day08::pt1, day08::pt2)?,
		9 => run(&input, day09::pt1, day09::pt2)?,
		10 => run(&input, day10::pt1, day10::pt2)?,
		11 => run(&input, day11::pt1, day11::pt2)?,
		12 => run(&input, day12::pt1, day12::pt2)?,
		13 => run(&input, day13::pt1, day13::pt2)?,
		14 => run(&input, day14::pt1, day14::pt2)?,
		15 => run(&input, day15::pt1, day15::pt2)?,
		16 => run(&input, day16::pt1, day16::pt2)?,
		17 => run(&input, day17::pt1, day17::pt2)?,
		18 => run(&input, day18::pt1, day18::pt2)?,
		19 => run(&input, day19::pt1, day19::pt2)?,
		20 => run(&input, day20::pt1, day20::pt2)?,
		21 => run(&input, day21::pt1, day21::pt2)?,
		22 => run(&input, day22::pt1, day22::pt2)?,
		23 => run(&input, day23::pt1, day23::pt2)?,
		24 => run(&input, day24::pt1, day24::pt2)?,
		25 => run(&input, day25::pt1, day25::pt2)?,
		_ => unreachable!(),
	}

	Ok(())
}

fn get_input(day: isize) -> anyhow::Result<String> {
	let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let filename = format!("{:0>2}", day);
	let input_path = directory.join(format!("input/{}.txt", filename));

	Ok(read_to_string(input_path)?)
}

fn run<F1, F2, T1, T2>(input: &str, pt1: F1, pt2: F2) -> anyhow::Result<()>
where
	F1: Fn(&str) -> anyhow::Result<T1>,
	F2: Fn(&str) -> anyhow::Result<T2>,
	T1: std::fmt::Display,
	T2: std::fmt::Display,
{
	let input = input.trim();
	let now = std::time::Instant::now();

	let pt1 = pt1(input);
	let pt2 = pt2(input);

	let elapsed = now.elapsed();

	println!("Part one: {}", pt1?);
	println!("Part two: {}", pt2?);
	println!("Duration: {:?}", elapsed);

	Ok(())
}
