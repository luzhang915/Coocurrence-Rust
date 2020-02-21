use std::fs;
use std::collections::HashMap;
use std::vec::Vec;
//use std::thread;
//use std::time::Duration;
mod coocurrence_char;
mod coocurrence_words;
mod gen_freq_groups;


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

// TOPK
fn topk() -> std::io::Result<()> {
    let dataset = "SherlockHolmes.txt";
    let k = 10;
    let data = fs::read_to_string(dataset).expect("Unable to read file");
    let data = data.trim();
    let mut top_k: Vec<String> = vec![];
    gen_freq_groups::find_top_k_freq(&data.to_string(), k, &mut top_k);
    println!("top {} words in {} are: {:?}", k, dataset, top_k);
    // top 10 words in SherlockHolmes.txt are: ["the", "I", "of", "and", "to", "a", "that", "in", "was", "it"]
    let mut path = String::from("results/top10/words_groups.txt");
    println!("file created at: {:?}", path);
    let mut file = fs::File::create(path)?;
    for i in 0..top_k.len()-2 {
        for j in i+1..top_k.len()-1 {
            for k in j+1..top_k.len() {
                let mut set = vec![top_k[i].clone(), top_k[j].clone(), top_k[k].clone()];
                let mut file_name = String::from("top10/");
                file_name.push_str(set[0].as_str());
                file_name.push('-');
                file_name.push_str(set[1].as_str());
                file_name.push('-');
                file_name.push_str(set[2].as_str());
                file_name.push_str(".txt ");
                let output = file_name.as_bytes();
                file.write_all(output)?;
                let err = coocurrence_words::coocurrence_all(&data.to_string(), set, 2000);
                println!("{:?}",err);
            }
        }
    }

    Ok(())
}

fn basic() {
    let data = fs::read_to_string("SherlockHolmes.txt").expect("Unable to read file");
    let data = data.trim();
    coocurrence_words::coocurrence_all(&data.to_string(), vec!["dear".to_string(), "Watson".to_string(), "I".to_string()], 10000);
//	coocurrence_char::coocurrence_all(&data.to_string(), vec!['A', 'B', 'C'], 10000);
}


fn main() {
    basic();
}


// randomly gen word group of size k e.g. (abcd) (choose unique k words from trace
// x axis: itemset size yaxis: time to update the histogram
// x axis: word group size y axis: time
                        // y axis memory
//rndom word set
// larger itemset size?

// more occ as win goes up, derivitive ? line goes and down
// original + 2nd deriv both should be like bellshape

