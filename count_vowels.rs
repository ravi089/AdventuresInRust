// Count vowels in a string.

fn count_vowels(data: &str) -> i32 {
	let mut result = 0;
	for c in data.to_string().chars().flat_map(|c| c.to_lowercase()) {
		result += match c {
			'a'|'e'|'i'|'o'|'u' => {
				1
			}
			_ => 0
		}
	}
	result
}

fn main(){
	println!("{}", count_vowels("geeksforgeeks portal"));
}
