#[cfg(test)]
mod tests {
    use crate::Fibonacci;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
        assert_eq!(fib.next(), Some(34));
    }

    #[test]
    fn test_fibonacci_overflow() {
        let mut fib = Fibonacci::new();
        // Consume the iterator until overflow (this is just an approximation)
        for _ in 0..100 {
            fib.next();
        }
        assert_eq!(fib.next(), None); // Expect None after overflow
    }

    #[test]
    fn test_fibonacci_initial_values() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
    }
}
