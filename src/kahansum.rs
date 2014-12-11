// Implements http://rosettacode.org/wiki/Kahan_summation


extern crate core;
use core::num::FromStrRadix;

fn find_max(lst: &Vec<f32>) -> Option<f32> {
    let mut max = None;
    for i in lst.iter() {
        max = match max {
            None => Some(*i),
            Some(m) if *i > m => Some(*i),
            _ => max
        }
    }
    max
}

fn with_bits(val: f32, digits: uint) -> f32 {
    let num = std::f64::to_str_digits(val as f64, digits);
    let res = FromStrRadix::from_str_radix(num.as_slice(), 10).unwrap();
    res
}

fn kahan_sum(lst: &Vec<f32>) -> Option<f32> {
    let mut sum = 0.0f32;
    let mut c = 0.0f32;
    for i in lst.iter() {
        let y = *i - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    Some(with_bits(sum, 5))
}


fn all_sums(vec: &Vec<f32>) -> Vec<f32> {
    let mut res = Vec::new();
    let mut perms = vec.permutations();
    loop {
        let v = perms.next();
        match v {
            Some(_v) =>  {
                let mut sum = 0.0f32;
                for e in _v.iter() {
                    sum += with_bits(*e, 5);
                }
                res.push(with_bits(sum, 5));
            }
            None => break
        }
    }
    res
}


fn main() {
    let v = vec![1.0f32, 2.0, 3.0];
    let res = find_max(&v);
    assert!(res == Some(3.0f32));
    test_kahansum();
}

fn test_kahansum() {
    let v = vec![10000.0f32, 3.14159, 2.71828];
    let sums = all_sums(&v);
    let res = kahan_sum(&v);
    let max = find_max(&sum).unwrap();
    assert!(max, res);
}
