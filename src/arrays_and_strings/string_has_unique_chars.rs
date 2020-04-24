fn run(s: &str) -> bool {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(run("qwerty"), true);
        assert_eq!(run("1234"), true);
        assert_eq!(run("!@#$"), true);
        assert_eq!(run("asdfa"), false);
    }
}
