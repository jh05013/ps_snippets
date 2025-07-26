//! Explanation

use std::ops::*;

pub fn partition_number<T>(n: usize) -> Vec<T>
where
    T: From<u32> + Clone + AddAssign + SubAssign,
{
    let mut p = vec![T::from(1)];
    for i in 1..=n {
        let mut ans = T::from(0);
        for k in 1.. {
            let Some(j) = i.checked_sub(k * (3*k-1)/2) else { break; };
            if k % 2 == 1 {
                ans += p[j].clone();
            } else {
                ans -= p[j].clone();
            }
            let Some(j) = j.checked_sub(k) else { break; };
            if k % 2 == 1 {
                ans += p[j].clone();
            } else {
                ans -= p[j].clone();
            }
        }
        p.push(ans);
    }
    p
}

#[cfg(test)]
mod tests {
    extern crate modint;
    use self::modint::Modint;

    use crate::partition_number;

    #[test]
    fn test_partition_number() {
        let p = partition_number::<u32>(0);
        assert_eq!(p, vec![1]);
        let p = partition_number::<u32>(1);
        assert_eq!(p, vec![1, 1]);
        let p = partition_number::<u32>(10);
        assert_eq!(p, vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42]);
        let p = partition_number::<Modint<100000>>(100000);
        assert_eq!(p.len(), 100001);
        assert_eq!(p[99997], 29176u32.into());
        assert_eq!(p[99998], 19485u32.into());
        assert_eq!(p[99999], 26875u32.into());
        assert_eq!(p[100000], 98519u32.into());
    }
}
