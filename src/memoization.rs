fn simple_memoize(f: impl Fn(i32) -> i32) -> impl FnMut(i32) -> i32 {
    let mut mem = None;
    move |x| {
        if let Some((arg, res)) = mem {
            if arg == x {
                return res;
            }
        }
        let res = f(x);
        mem = Some((x, res));
        res
    }
}

fn memoize<T, U>(f: impl Fn(T) -> U) -> impl FnMut(T) -> U
where
    T: PartialEq + Clone,
    U: Clone,
{
    let mut mem: Option<(T, U)> = None;
    move |x| {
        if let Some((arg, res)) = &mem {
            if *arg == x {
                return res.clone();
            }
        }
        let res = f(x.clone());
        mem = Some((x, res.clone()));
        res
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};
    use super::*;
    #[test]
    fn test_simple_memoize() {
        let mut memoized = simple_memoize(|x| x * x);
        assert_eq!(16, memoized(4));
    }
    #[test]
    fn test_memoize() {
        let mut memoized = memoize(|x| {
            thread::sleep(time::Duration::from_secs(2));
            x * x
        });
        assert_eq!(16, memoized(4));
    }
}
