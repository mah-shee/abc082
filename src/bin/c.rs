#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;
    for i in a.iter() {
        map.entry(*i).and_modify(|e| *e += 1).or_insert(1);
        if map.get(&*i).unwrap() > i {
            ans += 1;
        }
    }
    for (i, v) in map.iter() {
        if i > v {
            ans += v;
        }
    }
    println!("{}", ans);
}
