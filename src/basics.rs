/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    let mut result = -1;
    let mut index = 0;
    if n > 0 {
        result += 1;
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
    let mut flag = false;
    if target.len() == 0 {
        flag = true;
    } else {
        for i in target.iter() {
            flag = false;
            for j in set.iter() {
                if i == j {
                    flag = true;
                }
            }
            if flag == false {
                return false;
            }
        }
    }
    return flag;
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    if ls.len() == 0 {
        return None;
    } else {
        let mean = ls.iter().fold(0.0, |acc, x| (acc + x));
        return Some(mean / (ls.len() as f64));
    }
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array

    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    if ls.len() > 0 {
        let mut len = (ls.len() - 1) as i32;
        let mut num = 0;
        let base: i32 = 2;
        for i in ls.iter() {
            num += i * base.pow(len as u32);
            len -= 1;
        }
        return num;
    } else {
        return 0;
    }
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut x = n;
    for i in 2..=(n / 2) {
        while x % i == 0 {
            result.push(i);
            x /= i;
        }
    }
    if result.is_empty() {
        result.push(n)
    };
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

    if lst.len() > 0 {
        let first = vect.remove(0);
        vect.push(first);
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
pub fn longest_sequence(s: &str) -> Option<&str> {
    if s.len() == 0 {
        return None;
    } else {
        let mut result = "";
        let mut length = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                if s[i..i + 1] == s[j..j + 1] {
                    if j - i + 1 > length {
                        length = j - i + 1;
                        result = &s[i..(j + 1)];
                    }
                } else {
                    break;
                }
            }
        }
        return Some(result);
    }
}
