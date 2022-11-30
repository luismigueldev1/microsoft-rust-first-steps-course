pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2i32));
    }

    #[test]
    #[should_panic]
    fn is_false_when_odd() {
        assert!(is_even(3i32));
    }
}
