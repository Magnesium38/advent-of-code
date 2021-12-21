#[macro_use]
extern crate prettytable;

use prettytable::{Cell, Row, Table};
use std::time::Duration;

const RUNS: u32 = 3;

fn main() -> anyhow::Result<()> {
	let jobs = [
		(wrap_fn(&day01::pt1), wrap_fn(&day01::pt2), "01"),
		(wrap_fn(&day02::pt1), wrap_fn(&day02::pt2), "02"),
		(wrap_fn(&day03::pt1), wrap_fn(&day03::pt2), "03"),
		(wrap_fn(&day04::pt1), wrap_fn(&day04::pt2), "04"),
		(wrap_fn(&day05::pt1), wrap_fn(&day05::pt2), "05"),
		(wrap_fn(&day06::pt1), wrap_fn(&day06::pt2), "06"),
		(wrap_fn(&day07::pt1), wrap_fn(&day07::pt2), "07"),
		(wrap_fn(&day08::pt1), wrap_fn(&day08::pt2), "08"),
		(wrap_fn(&day09::pt1), wrap_fn(&day09::pt2), "09"),
		(wrap_fn(&day10::pt1), wrap_fn(&day10::pt2), "10"),
		(wrap_fn(&day11::pt1), wrap_fn(&day11::pt2), "11"),
		(wrap_fn(&day12::pt1), wrap_fn(&day12::pt2), "12"),
		(wrap_fn(&day13::pt1), wrap_fn(&day13::pt2), "13"),
		(wrap_fn(&day14::pt1), wrap_fn(&day14::pt2), "14"),
		(wrap_fn(&day15::pt1), wrap_fn(&day15::pt2), "15"),
		(wrap_fn(&day16::pt1), wrap_fn(&day16::pt2), "16"),
		(wrap_fn(&day17::pt1), wrap_fn(&day17::pt2), "17"),
		(wrap_fn(&day18::pt1), wrap_fn(&day18::pt2), "18"),
		(wrap_fn(&day19::pt1), wrap_fn(&day19::pt2), "19"),
		(wrap_fn(&day20::pt1), wrap_fn(&day20::pt2), "20"),
		(wrap_fn(&day21::pt1), wrap_fn(&day21::pt2), "21"),
		(wrap_fn(&day22::pt1), wrap_fn(&day22::pt2), "22"),
		(wrap_fn(&day23::pt1), wrap_fn(&day23::pt2), "23"),
		(wrap_fn(&day24::pt1), wrap_fn(&day24::pt2), "24"),
		(wrap_fn(&day25::pt1), wrap_fn(&day25::pt2), "25"),
	];

	let results = jobs
		.iter()
		.map(|(pt1, pt2, day)| {
			let input =
				std::fs::read_to_string(format!("input/{}.txt", day)).unwrap_or_else(|_| "".to_string());
			let input = input.trim();

			Ok((
				day,
				benchmark_function(&pt1, input)?,
				benchmark_function(&pt2, input)?,
			))
		})
		.collect::<anyhow::Result<Vec<_>>>()?;

	let mut table = Table::new();
	table.set_titles(row![
		c =>
		"Day",
		"Min (1)",
		"Max (1)",
		"Avg (1)",
		"Percent (1)",
		"Min (2)",
		"Max (2)",
		"Avg (2)",
		"Percent (2)",
	]);

	let pt1_total = results
		.iter()
		.map(|(_, (_, _, pt1_avg), _)| pt1_avg)
		.sum::<u128>();

	let pt2_total = results
		.iter()
		.map(|(_, _, (_, _, pt2_avg))| pt2_avg)
		.sum::<u128>();

	for result in results
		.iter()
		.map(
			|&(day, (pt1_min, pt1_max, pt1_avg), (pt2_min, pt2_max, pt2_avg))| {
				Ok(vec![
					Cell::new(&day.to_string()),
					make_cell(pt1_min),
					make_cell(pt1_max),
					make_cell(pt1_avg),
					{
						let mut cell = Cell::new(&format!(
							"{:.4}%",
							pt1_avg as f64 / pt1_total as f64 * 100.0,
						));

						cell.align(prettytable::format::Alignment::RIGHT);

						cell
					},
					make_cell(pt2_min),
					make_cell(pt2_max),
					make_cell(pt2_avg),
					{
						let mut cell = Cell::new(&format!(
							"{:.4}%",
							pt2_avg as f64 / pt2_total as f64 * 100.0,
						));

						cell.align(prettytable::format::Alignment::RIGHT);

						cell
					},
				])
			},
		)
		.collect::<anyhow::Result<Vec<_>>>()?
	{
		table.add_row(Row::new(result));
	}

	table.add_row(row![
		"Total",
		"",
		"",
		make_cell(pt1_total),
		"",
		"",
		"",
		make_cell(pt2_total),
		"",
	]);

	table.printstd();

	Ok(())
}

fn wrap_fn<T>(
	f: &dyn Fn(&str) -> anyhow::Result<T>,
) -> Box<dyn Fn(&str) -> anyhow::Result<()> + '_> {
	Box::new(|input: &str| {
		f(input)?;

		Ok(())
	})
}

fn make_cell(v: u128) -> Cell {
	let mut cell = Cell::new(&format!("{:.4}ms", (v as f64) / 1_000_000.0));

	cell.align(prettytable::format::Alignment::RIGHT);

	cell
}

fn benchmark_function<T>(
	f: &dyn Fn(&str) -> Result<T, anyhow::Error>,
	input: &str,
) -> anyhow::Result<(u128, u128, u128)> {
	let durations = (0..RUNS)
		.map(|_| {
			let now = std::time::Instant::now();

			f(input)?;

			Ok(now.elapsed())
		})
		.collect::<anyhow::Result<Vec<_>>>()?;

	let min = durations.iter().min().unwrap();
	let max = durations.iter().max().unwrap();
	let avg = durations.iter().sum::<Duration>() / RUNS;

	Ok((min.as_nanos(), max.as_nanos(), avg.as_nanos()))
}
