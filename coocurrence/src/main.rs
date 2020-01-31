use std::fs;
use std::collections::HashMap;
use std::vec::Vec;
//use std::thread;
//use std::time::Duration;
mod coocurrence_char;
mod coocurrence_words;


//fn main() {
//	let data = fs::read_to_string("test.txt").expect("Unable to read file");
//	let data = data.trim();
////	coocurrence_words::coocurrence_all(&data.to_string(), vec!["Sherlock".to_string(), "Watson".to_string(), "said".to_string()], 10000);
////	coocurrence_char::coocurrence_all(&data.to_string(), vec!['A', 'B', 'C'], 10000);
//    let mut sets: Vec<Vec<char>> = Vec::new();
//    sets.push(vec!['A', 'B', 'C']);
//    sets.push(vec!['Q', 'A', 'B']);
//
//    let mut children = vec![];
//
//    for set in sets {
//        children.push(thread::spawn(move || {
//            coocurrence_char::coocurrence_all(&data.to_string(), set, 10000);
//            thread::sleep(Duration::from_millis(1));
//        }));
//
//    }
//    for child in children {
//        child.join().unwrap();
//    }
//
//}

fn main() {
    let data = fs::read_to_string("SherlockHolmes.txt").expect("Unable to read file");
    let data = data.trim();
    coocurrence_words::coocurrence_all(&data.to_string(), vec!["dear".to_string(), "Watson".to_string(), "I".to_string()], 10000);
//	coocurrence_char::coocurrence_all(&data.to_string(), vec!['A', 'B', 'C'], 10000);
}
