// Implements http://rosettacode.org/wiki/Kahan_summation

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
    let num = std::f32::to_str_digits(val, digits);
    let res: f32 = from_str(num.as_slice()).unwrap();
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
    Some(with_bits(sum, 1))
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
                    sum += with_bits(*e, 1);
                }
                res.push(with_bits(sum, 1));
            }
            None => break
        }
    }
    res
}

#[cfg(not(test))]
fn main() {
    let v = vec![10000.0f32, 3.14159, 2.71828];
    let sums = all_sums(&v);
    let res = kahan_sum(&v).unwrap();
    let max = find_max(&sums).unwrap();
    println!("max: {} res: {}", max, res);
}

#[test]
fn test_kahansum() {
    let v = vec![10000.0f32, 3.14159, 2.71828];
    let sums = all_sums(&v);
    let res = kahan_sum(&v).unwrap();
    let max = find_max(&sums).unwrap();
    assert!(max < res);
}

#[test]
fn test_withbits() {
    let v = 3.123345f32;
    let res = with_bits(v, 3);
    assert!(res == 3.123f32);
}
