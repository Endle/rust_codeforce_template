#[inline(always)]
#[allow(dead_code)]
pub fn multi_mod(x:i32, y:i32, modulo:i32) -> i32 {
    let mid:i64 = (x as i64) * (y as i64);
    let mid = mid % (modulo as i64);
    return mid as i32;
}

#[allow(dead_code)]
pub fn pow_mod(base: i32, exp:u32, modulo:i32) -> i32 {
    let mut answer:i64 = 1;
    if exp > 10 {
        //Todo: Move to non-recursive
        let sub_exp = exp / 2;
        let mut p:i64 = pow_mod(base, sub_exp, modulo) as i64;
        p = p as i64 * p as i64 % modulo as i64;
        if exp % 2 == 1 {
            p = p * base as i64 % modulo as i64;
        }
        return p as i32;
    }
    for _ in 0..exp{
        answer *= base as i64;
        answer %= modulo as i64;
    }
    answer as i32
}

//should be generic
#[inline(always)]
#[allow(dead_code)]
pub fn in_closed_range(left: i32, right:i32, x:i32) -> bool {
    left <= x && x <= right
}
#[inline(always)]
#[allow(dead_code)]
pub fn in_closed_range_reversible(left: i32, right:i32, x:i32) -> bool {
    match left.cmp(&right) {
        std::cmp::Ordering::Less => in_closed_range(left, right, x),
        std::cmp::Ordering::Equal => x == left,
        std::cmp::Ordering::Greater => in_closed_range(right, left, x),
    }
}

pub fn select_lowest_kth_bit(x: u32, b: u32) -> u32 {
    let result = (x >> b) & 1;
    assert!(result == 0 || result == 1);
    result
}
//Needs mature
#[allow(dead_code)]
fn calc_combination_with_mod(a: i32, b: i32, modulo: i32) -> i32 {
    // https://www.geeksforgeeks.org/compute-ncr-p-set-3-using-fermat-little-theorem/
    let mut num:i64 = 1;
    let mut den:i64 = 1;
    for i in 0..b {
        num = num * (a - i) as i64 % modulo as i64;
        den = den * (i + 1) as i64 % modulo as i64;
    }

    let answer = num * pow_mod(den as i32, modulo as u32 - 2, modulo) as i64;
    let answer = answer % modulo as i64;
    answer as i32
}