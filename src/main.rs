use std::{
	env::current_dir,
	fs::{create_dir_all, read_to_string, File},
	io::Write,
};

fn main() -> anyhow::Result<()> {
	dotenv::dotenv()?;

	let session = std::env::var("AOC_SESSION")
		.map_err(anyhow::Error::from)
		.and_then(|s| {
			if !s.is_empty() {
				Ok(s)
			} else {
				Err(anyhow::anyhow!("AOC_SESSION is not set"))
			}
		})?;

	let day = std::env::args()
		.nth(1)
		.ok_or(anyhow::anyhow!("missing day"))?;
	let filename = format!("{:0>2}", day);

	create_dir_all(current_dir()?.join(format!("day{}/src/", filename)))?;
	copy_file(
		"template/src/main.rs",
		&format!("day{}/src/main.rs", filename),
		&filename,
	)?;
	copy_file(
		"template/src/ADVENTDAY.rs",
		&format!("day{}/src/{}.rs", filename, filename),
		&filename,
	)?;
	copy_file(
		"template/Cargo.toml",
		&format!("day{}/Cargo.toml", filename),
		&filename,
	)?;

	let input_path = current_dir()?.join(format!("input/{}.txt", filename));
	if !input_path.exists() {
		let client = reqwest::blocking::Client::new();
		let mut response = client
			.get(format!("https://adventofcode.com/2021/day/{}/input", day))
			.header("Cookie", format!("session={}", session))
			.send()?;

		if !response.status().is_success() {
			return Err(anyhow::anyhow!("failed to get input"));
		}

		let mut file = std::fs::File::create(input_path)?;
		response.copy_to(&mut file)?;
		println!("Downloaded input");
	}

	Ok(())
}

fn copy_file(src: &str, dst: &str, replace_with: &str) -> anyhow::Result<()> {
	let src_path = current_dir()?.join(src);
	let dst_path = current_dir()?.join(dst);

	if !dst_path.exists() {
		let contents = read_to_string(src_path)?.replace("ADVENTDAY", replace_with);

		File::create(dst_path)?.write_all(contents.as_bytes())?;
	}

	Ok(())
}
