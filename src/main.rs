// Please Read [Rule about third-party code is changing](https://codeforces.com/blog/entry/8790)
extern crate my_lib;
use my_lib::{read, read_ivec};
use my_lib::pr;
use my_lib::memorize;
use my_lib::nd;
use my_lib::multi_queue;
use my_lib::algo;



// Currently bundler https://github.com/Endle/rust-bundler/tree/codeforce doesn't support use *

fn solve() {
}


// Below is template
fn main() {
    let testcases: i32 = read!();
    for _ in 0..testcases { solve(); }
}
