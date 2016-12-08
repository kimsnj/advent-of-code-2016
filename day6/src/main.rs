use std::io::BufRead;
use std::collections::HashMap;


fn update_freqencies(freq: &mut Vec<HashMap<char, i32>>, line: &str) {
    assert!(freq.len() == line.len());
    for (f, c) in freq.iter_mut().zip(line.chars()) {
        *f.entry(c).or_insert(0) += 1;
    }
}

fn max(freq: &HashMap<char, i32>) -> char {
    let (c, _) = freq.iter().fold((' ', -1), |(max_c, max_count), (&c, &count)| {
        if count > max_count {
            (c, count)
        } else {
            (max_c, max_count)
        }
    });
    c
}

fn min(freq: &HashMap<char, i32>) -> char {
    let (c, _) = freq.iter().fold((' ', i32::max_value()), |(max_c, max_count), (&c, &count)| {
        if count > max_count {
            (max_c, max_count)
        } else {
            (c, count)
        }
    });
    c
}

fn main() {
    let f = std::fs::File::open("input").expect("Unable to open input file");
    let mut freqs = vec![HashMap::new(); 8];

    for l in std::io::BufReader::new(f).lines() {
        update_freqencies(&mut freqs, l.unwrap().trim());
    }

    println!("Most frequent: {:?}", freqs.iter().map(max).collect::<String>());
    println!("Least frequent: {:?}", freqs.iter().map(min).collect::<String>());
}
