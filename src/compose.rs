fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compose() {
        let comp = compose(|n| n * n, |n| n + 1);
        assert_eq!(17, comp(4));
    }
}
