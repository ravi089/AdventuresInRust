// Check if two strings are anagrams.

fn sort_data(data: &str) -> String {
	let mut chars: Vec<_> = data.to_string()
		.chars()	// get the iterator
		.flat_map(|c| c.to_lowercase())
		.collect(); 
	chars.sort();
	chars
		.iter()
		.collect::<String>()
		.chars()
		.collect::<String>()
}

fn anagram(str1: &str, str2: &str) -> bool {
	
	let str1 = sort_data(&str1);
	let str2 = sort_data(&str1);
	
	(str1 == str2)
}

fn main(){
	println!("{}", anagram("stREsSeD", "DeSSertS"));
}
