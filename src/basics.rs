/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    let mut result = -1;
    let mut index = 0;
    if n > 0 {
        loop {
            result += index;
            if index == n {
                break;
            };
            index += 1;
        }
        result
    } else {
        result
    }
}

/**
    Returns the number of elements in the list that
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut count = 0;
    for i in ls.iter() {
        if i >= &s && i <= &e {
            count += 1
        }
    }
    return count;
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    let mut result = true;
    for i in set.iter() {
        for j in target.iter() {
            if i != j {
                result = false;
                break;
            }
        }
    }
    return result;
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    if ls.len() == 0 {
        return None;
    } else {
        let mean = ls.iter().fold(0.0, |acc, x| (acc + x) / 2.0);
        return Some(mean);
    }
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array

    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    let mut len = ls.len() - 1;
    let mut num = 0;
    for i in ls.iter() {
        num += i * 2_i32.pow(len as u32);
        len -= 1;
    }
    return num;
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut x = n;
    for i in 2..=((n as f32).sqrt() as u32) {
        while x % i == 0 {
            result.push(i);
            x /= i;
        }
    }
    return result;
}

/**
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them,
    so the first becomes the last, the second becomes first, and so on.

    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut vect = lst.to_vec();
    let f = vect.pop();
    match f {
        Some(f) => vect.push(f),
        None => (),
    }
    return vect;
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation

    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    let mut result = false;
    if target.len() <= s.len() {
        let mut i = 0;
        while i <= (s.len() - target.len()) {
            if (&s[i..(i + target.len())]) == target {
                result = true;
                break;
            }
            i += 1;
        }
    }
    return result;
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Vec<char>{
    let mut ret = Vec::new();
    let mut i = 1;
    match s.chars().nth(0) {
        Some(x) => ret.push(x),
        None => ()
    }
    while i <= s.len() {
        if s.chars().nth(i) == ret.pop() {
            match s.chars().nth(i) {
                Some(x) => ret.push(x),
                None => (),
            }
        }
        i += 1;
    }
    return ret;
}
