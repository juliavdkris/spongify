use std::io::{self, BufRead};

fn spongify_char(c: char) -> String {
	match rand::random::<bool>() {
		true => c.to_uppercase().to_string(),
		false => c.to_lowercase().to_string(),
	}
}

fn main() -> io::Result<()> {
	let stdin = io::stdin();

	for line in stdin.lock().lines() {
		let line = line?;

		let sponged: String = line.chars().map(spongify_char).collect();
		println!("{}\n", sponged);
	}
	Ok(())
}
