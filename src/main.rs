use std::env::current_dir;

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

	let src_path = current_dir()?.join(format!("src/bin/{}.rs", filename));
	if !src_path.exists() {
		std::fs::copy("template.rs.stub", src_path)?;
		println!("Created source stub");
	}

	Ok(())
}
