fn id<T>(x: T) -> T {
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_id() {
        assert_eq!(42, id(42));
        assert_eq!("coucou", id("coucou"));
    }
}
