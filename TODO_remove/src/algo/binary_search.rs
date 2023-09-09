pub trait BinarySearch where Self: Sized {
	fn midpoint(a: Self, b: Self) -> Option<Self>;
}

pub fn binary_search<T, R>(f: impl Fn(T) -> Option<R>, l: T, r: T)
	-> std::result::Result<(T, R), (Option<R>, Option<R>)>
where T: BinarySearch + Clone {
	let (mut yes, mut no): ((T, R), T);
	match (f(l.clone()), f(r.clone())) {
		(Some(res), None) => { yes = (l, res); no = r; }
		(None, Some(res)) => { yes = (r, res); no = l; }
		(a, b) => { return Err((a, b)); }
	}
	loop {
		let Some(mid) = T::midpoint(yes.0.clone(), no.clone()) else { return Ok(yes); };
		if let Some(res) = f(mid.clone()) { yes = (mid, res); }
		else { no = mid; }
	}
}

// TODO prevent overflow
macro_rules! impl_bs_int {
	($($T:ty) *) => { $( impl BinarySearch for $T {
		fn midpoint(a: Self, b: Self) -> Option<Self> {
			let m = (a+b)/2;
			if a == m || b == m { None } else { Some(m) }
		} } )*
	};
}
impl_bs_int!(i8 i16 i32 i64 u8 u16 u32 u64 usize);

impl BinarySearch for f64 {
	fn midpoint(a: Self, b: Self) -> Option<Self> {
		let m = (a+b)/2.;
		if (a-m).abs() <= 1e-12 { None } else { Some(m) }
	}
}
