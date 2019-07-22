// Run length encoding of string.

fn get_count(n: u32) -> String {
	match n {
		1 => "".into(),
		_ => n.to_string()
	}
}

fn run_length_encoding(data: &str) -> String {
	let mut encoded = String::new();
	let mut count = 1;
	let mut current= data.chars().nth(0).unwrap();
	
	for c in data.chars().skip(1) {
		if c == current {
			count += 1;
		} else {
			encoded.push(current);
			encoded.push_str(&get_count(count));
			current = c;
			count = 1;
		}
	}
	encoded.push(current);
	encoded.push_str(&get_count(count));
	
	encoded
}

fn run_length_decoding(data: &str) -> String {
	let mut current = String::new();
	let mut decoded= String::new();
	
	for c in data.chars() {
		if c.is_numeric() {
			let digit: usize = c.to_string().parse().unwrap_or(1);
			decoded.push_str(&current.repeat(digit));
			current.clear();
		} else {
			if !current.is_empty() {
				decoded.push_str(&current);
				current.clear();
			}
			current.push(c);
		}
	}
	decoded
}

fn main(){
	
	let mut data = run_length_encoding("wwwwaaadexxxxxx");
	println!("{}", data);
	data = run_length_decoding(&data);
	println!("{}", data);
}
