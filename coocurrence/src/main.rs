use std::fs;
use std::collections::HashMap;
use std::vec::Vec;
mod coocurrence_char;
mod coocurrence_words;


fn main() {
	let data = fs::read_to_string("test.txt").expect("Unable to read file");
	let data = data.trim();
//	coocurrence_words::coocurrence_all(&data.to_string(), vec!["Sherlock".to_string(), "Watson".to_string(), "said".to_string()], 10000);
	coocurrence_char::coocurrence_all(&data.to_string(), vec!['A', 'B', 'C'], 10000);
}
