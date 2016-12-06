extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::BTreeMap;

fn main() {
    let mut sh = Md5::new();
    let mut code: BTreeMap<char, char> = BTreeMap::new();
    for i in 1.. {
        sh.input_str(&format!("cxdnnyjw{}", i));
        let hash = sh.result_str();
        let (zeros, rest) = hash.split_at(5);
        if zeros == "00000" {
            let mut chars = rest.chars();
            let pos = chars.next().unwrap();
            if pos.is_numeric() && pos < '8' && !code.contains_key(&pos){
                code.insert(pos, chars.next().unwrap());
            }
            if code.len() == 8 {
                break;
            }
        }
        sh.reset();
    }
    for (_, c) in code {
        print!("{}", c);
    }
}
