mod grid;

pub use grid::{Grid, Neighbors, Iter};

#[macro_export]
macro_rules! main {
	() => {
		fn main() -> anyhow::Result<()> {
			let mut filepath = std::env::current_dir()?;
			filepath.push(file!().replace(".rs", ".txt").replace("src/bin/", "input/"));

			let input = std::fs::read_to_string(filepath)?;
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
	};
}

#[macro_export]
macro_rules! test {
	($input: expr, $pt1: expr, $pt2: expr) => {
		#[cfg(test)]
		mod tests {
			use super::*;

			fn prepare_input<'a>(input: &'a str) -> String {
				input
					.trim()
					.lines()
					.map(|line| line.trim())
					.collect::<Vec<_>>()
					.join("\n")
			}

			#[test]
			fn test_pt1() {
				assert_eq!(pt1(&prepare_input($input)).unwrap(), $pt1);
			}

			#[test]
			fn test_pt2() {
				assert_eq!(pt2(&prepare_input($input)).unwrap(), $pt2);
			}
		}
	};
}

#[macro_export]
macro_rules! problem {
	($test_input:expr, $pt1_answer:expr, $pt2_answer:expr, $(,)*) => {
		advent::main!();
		advent::test!($test_input, $pt1_answer, $pt2_answer);
	};
}
