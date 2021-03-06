use std::collections::HashMap;
use std::vec::Vec;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn coocurrence_all(trace: &String, set: Vec<String>, window_bound: u32) -> std::io::Result<()> {
//    let content = include_str!("not_existing_file.txt");
//    println!("{}", content);
    let mut path = String::from("results/top10/");
    path.push_str(set[0].as_str());
    path.push('-');
    path.push_str(set[1].as_str());
    path.push('-');
    path.push_str(set[2].as_str());
    path.push_str(".txt");

    println!("file created at: {:?}", path);
    let mut file = fs::File::create(path)?;


    let mut histogram:HashMap<u32,i32> = HashMap::new();
    let mut reuses:HashMap<String,u32> = HashMap::new();
    let mut stack:Vec<String> = Vec::new();

    for c in &set {
        reuses.insert(c.clone(),0);
        stack.push(c.clone());
    }

    let mut word_trace = vec![];
    for word in trace.split(|c:char| !c.is_alphanumeric()) {
        if !word.is_empty() {
            word_trace.push(word);
        }
    }
    let mut i = 0;
    for w in word_trace.clone() {
        if w.is_empty() {
            continue;
        }
        if set.contains(&w.to_string()) {
            if stack[set.len()-1].eq(w) {
                let second_from_bottom = i as u32 - *reuses.get(&stack[set.len()-2]).unwrap();
                let bottom = i as u32 - *reuses.get(&stack[set.len()-1]).unwrap();
                histogram.insert(second_from_bottom, match histogram.get(&second_from_bottom)
                    {None=>0,
                        Some(x)=>*x}-1);
                histogram.insert(bottom, match histogram.get(&bottom)
                    {None=>0,
                        Some(x)=>*x}+1);
            }

            reuses.insert(w.to_string(), i as u32+1);
            stack.retain(|x| !x.eq(w));
            stack.insert(0, w.to_string());
        }
        i += 1;
    }

    let i = word_trace.len();
    let bottom = i as u32-*reuses.get(&stack[set.len()-1]).unwrap();
    histogram.insert(bottom, match histogram.get(&bottom)
        {None=>0,
            Some(x)=>*x}+1);

    let mut count_1 = 0;
    let mut count_2 = 0;

    for i in (1 .. word_trace.len()+1).rev() {
        let num = match histogram.get(&(i as u32))
            {None=>0,
                Some(x)=>*x};

        count_1+=num;
        count_2+=num*(i as i32+1);

        if i as u32 <= window_bound {
//            println!("{} {}", i, (word_trace.len() as i32 -i as i32+1)-(count_2-i as i32*count_1));
            let mut s = "".to_string();
            let o = s + (i as u32).to_string().as_str() + " " + ((word_trace.len() as i32 -i as i32+1)-(count_2-i as i32*count_1)).to_string().as_str() + "\n";
            let output = o.as_bytes();
            file.write_all(output)?;
        }
    }

    Ok(())
}