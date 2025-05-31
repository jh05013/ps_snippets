//! Explanation

pub trait BinSearch: Sized {
    fn binary_search_min<F: Fn(&Self) -> bool>(pred: F, lo: Self, hi: Self) -> Option<Self>;
}

impl BinSearch for i64 {
    fn binary_search_min<F: Fn(&Self) -> bool>(
        pred: F,
        mut lo: Self,
        mut hi: Self,
    ) -> Option<Self> {
        let mut ans = None;
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if pred(&mid) {
                ans = Some(mid);
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        ans
    }
}
