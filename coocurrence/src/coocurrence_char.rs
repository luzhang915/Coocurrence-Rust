use std::collections::HashMap;
use std::vec::Vec;

pub fn coocurrence_all(trace : &String, set:Vec<char>, window_bound:u32) {
    let mut histogram:HashMap<u32,i32> = HashMap::new();
    let mut reuses:HashMap<char,u32> = HashMap::new();
    let mut stack:Vec<char> = Vec::new();

    for c in &set {
        reuses.insert(*c,0);
        stack.push(*c);
    }

    for (i,c) in trace.chars().enumerate() {
        println!("i: {}, c: {}", i, c);
        if set.contains(&c) {
            if stack[set.len()-1] == c{
                let second_from_bottom = i as u32-*reuses.get(&stack[set.len()-2]).unwrap();
                let bottom = i as u32-*reuses.get(&stack[set.len()-1]).unwrap();
                histogram.insert(second_from_bottom, match histogram.get(&second_from_bottom)
                    {None=>0,
                        Some(x)=>*x}-1);
                histogram.insert(bottom, match histogram.get(&bottom)
                    {None=>0,
                        Some(x)=>*x}+1);
            }

            reuses.insert(c, i as u32+1);
            stack.retain(|&x| x != c);
            stack.insert(0, c);
        }
    }

    //
    let i = trace.len();
    let bottom = i as u32-*reuses.get(&stack[set.len()-1]).unwrap();
    histogram.insert(bottom, match histogram.get(&bottom)
        {None=>0,
            Some(x)=>*x}+1);

    //


    let mut count_1 = 0;
    let mut count_2 = 0;

    for i in (1 .. trace.len()+1).rev() {
        let num = match histogram.get(&(i as u32))
            {None=>0,
                Some(x)=>*x};

        count_1+=num;
        count_2+=num*(i as i32+1);

        if i as u32 <= window_bound{
            println!("{}-{}", i, (trace.len() as i32 -i as i32+1)-(count_2-i as i32*count_1));
        }
    }
}