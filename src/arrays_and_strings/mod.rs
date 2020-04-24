use std::collections::HashMap;

fn is_unique(s: &str) -> bool {
    let mut alphabet_map = [false; 128];
    for c in s.chars().map(|c| c as usize) {
        if alphabet_map[c] {
            return false;
        } else {
            alphabet_map[c] = true;
        }
    };

    return true;
}

// assertion:
// s1 is a permutation of s2 if they contain the same letters in the same quantities
//
// notes:
// alternate implementation could leverage an array the size of the charset.
fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false
    }

    let mut char_map: HashMap<char, u8> = HashMap::new();
    for c in s1.chars() {
        let count = char_map.entry(c).or_insert(0);
        *count += 1;
    };

    for c in s2.chars() {
        let count = char_map.get_mut(&c);
        match count {
            Some(x) if *x > 0u8 => *x -= 1,
            _ => return false, // encountered a char not in the first string
        }
    };

    for (_, count) in char_map.iter() {
        if *count != 0u8 {
            return false; // char occurrences don't match
        }
    };

    true
}

// assertion:
// a string is a palindrome permutation if...
// 1) it has an even number of characters and each character occurs an even number of times
// 2) it has an odd number of characters and every character except one occurs an even number of
//    times
fn is_palindrome_permutation(s: &str) -> bool {
    let stripped_s: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
    let mut char_map: HashMap<char, i8> = HashMap::new();
    for c in &stripped_s {
        if c.is_whitespace() {
            continue
        }
        let count = char_map.entry(*c).or_insert(0);
        *count += 1;
    };

    let num_odd_counts = char_map
        .iter()
        .filter(|(_, c)| is_odd(c))
        .count();
    if num_odd_counts == 0 {
        return true;
    } else if num_odd_counts > 1 {
        return false;
    } else if num_odd_counts == 1 {
        return is_odd(&(stripped_s.len() as i8))
    } else {
        panic!("can't have negative count")
    }
}

fn is_odd(&x: &i8) -> bool {
    x % 2 != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert!(is_unique("qwerty"));
        assert!(is_unique("1234"));
        assert!(is_unique("!@#$"));
        assert!(!is_unique("asdfa"));
    }

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation("qwerty", "ytrewq"));
        assert!(is_permutation("qwerty", "twerqy"));
        assert!(!is_permutation("qwerty", "qwertyqwert"));
        assert!(!is_permutation("qwerty", "qwertt"));
        assert!(!is_permutation("qwerty", "asdfgh"));
    }

    #[test]
    fn test_is_palindrome_permutation() {
        assert!(is_palindrome_permutation("tacocat"));
        assert!(is_palindrome_permutation("taco cat"));
        assert!(!is_palindrome_permutation("tacodog"));
    }
}
