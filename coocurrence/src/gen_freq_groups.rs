use std::collections::HashMap;
use std::collections::LinkedList;
use std::vec::Vec;

// bucket sort
pub fn find_top_k_freq(trace: &String, k: u32, top_k: &mut Vec<String>) {
    // parse trace into words vec
    let mut word_trace = vec![];
    for word in trace.split(|c:char| !c.is_alphanumeric()) {
        if !word.is_empty() {
            word_trace.push(word);
        }
    }

    // count word frequency
    let mut word_count = HashMap::new();
    for w in word_trace.clone() {
        word_count.insert(w, match word_count.get(w) {
            Some(count) => *count,
            None => 0
        } + 1);
    }

    // push into buckets by frequency
    let mut buckets:Vec<LinkedList<String>> = vec![LinkedList::new(); word_trace.len()];
//    for i in 0..word_trace.len() {
//        let mut list = LinkedList::new();
//        buckets.push(list);
//    }
    for (word, count) in word_count {
//        println!("word: {}, count: {}", word, count);
        buckets[count].push_back(word.to_string());
    }

    // get top-k frequent elements
    let mut n = 0;
    for i in (0..buckets.len()).rev() {
        while !buckets[i].is_empty() {
            top_k.push(buckets[i].pop_front().unwrap().to_string());
            n += 1;
            if n>=k {break;}
        }
        if n>=k {break;}
    }

}