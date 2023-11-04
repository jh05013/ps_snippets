// https://github.com/Bubbler-4/rust-problem-solving/blob/main/competitive/src/competelib/fft.rs
pub mod ntt {
	pub mod mod998244353 {
		pub const MOD: usize = 998244353;
		pub const ROOTS: [usize; 24] = [1, 998244352, 911660635, 372528824, 929031873, 452798380, 922799308, 781712469, 476477967, 166035806, 258648936, 584193783, 63912897, 350007156, 666702199, 968855178, 629671588, 24514907, 996173970, 363395222, 565042129, 733596141, 267099868, 15311432];
		pub const INV: [usize; 24] = [1, 998244352, 86583718, 509520358, 337190230, 87557064, 609441965, 135236158, 304459705, 685443576, 381598368, 335559352, 129292727, 358024708, 814576206, 708402881, 283043518, 3707709, 121392023, 704923114, 950391366, 428961804, 382752275, 469870224];
		pub const INV_P2: [usize; 24] = [1, 499122177, 748683265, 873463809, 935854081, 967049217, 982646785, 990445569, 994344961, 996294657, 997269505, 997756929, 998000641, 998122497, 998183425, 998213889, 998229121, 998236737, 998240545, 998242449, 998243401, 998243877, 998244115, 998244234];
	}
	pub mod mod104857601 {
		pub const MOD: usize = 104857601;
		pub const ROOTS: [usize; 23] = [1, 104857600, 104847361, 76756981, 34399420, 93323136, 98667812, 78472926, 73653238, 33690314, 18773644, 4354736, 43120115, 62844082, 65430330, 80259084, 100680575, 81980263, 35912312, 18702539, 79427530, 98507391, 39193363];
		pub const INV: [usize; 23] = [1, 104857600, 10240, 83765945, 45929376, 19297248, 21338453, 99625490, 42994480, 83847972, 23338676, 18512281, 24489994, 82421973, 8903218, 45551298, 89241999, 59591738, 35844891, 72243308, 8583183, 71338971, 96987805];
		pub const INV_P2: [usize; 23] = [1, 52428801, 78643201, 91750401, 98304001, 101580801, 103219201, 104038401, 104448001, 104652801, 104755201, 104806401, 104832001, 104844801, 104851201, 104854401, 104856001, 104856801, 104857201, 104857401, 104857501, 104857551, 104857576];
	}
	pub use mod998244353::{MOD, ROOTS, INV, INV_P2};
	pub fn fft_opt(a: &mut [usize], invert: bool) {
		let n = a.len();
		let mut j = 0;
		for i in 1..n {
			let mut bit = n >> 1;
			while (j & bit) != 0 { j ^= bit; bit >>= 1; }
			j ^= bit;
			if i < j { a.swap(i, j); }
		}
		let mut len = 2;
		let mut idx = 1;
		while len <= n {
			let root = if invert { INV } else { ROOTS }[idx];
			for i in 0..n/len {
				let i = i * len;
				let mut w = 1;
				for j in 0..len/2 {
					let u = a[i+j]; let v = a[i+j+len/2] * w % MOD;
					a[i+j] = (u+v) % MOD; a[i+j+len/2] = (u+MOD-v) % MOD;
					w = w * root % MOD;
				}
			}
			len <<= 1; idx += 1;
		}
		if invert {
			let inv = INV_P2[n.trailing_zeros() as usize];
			for x in a { *x = *x * inv % MOD; }
		}
	}
	pub fn polymul(a: &[usize], b: &[usize]) -> Vec<usize> {
		let lens = (a.len() + b.len()).next_power_of_two();
		let mut fa = vec![0; lens];
		let mut fb = vec![0; lens];
		fa[..a.len()].copy_from_slice(a);
		fb[..b.len()].copy_from_slice(b);
		fft_opt(&mut fa, false);
		fft_opt(&mut fb, false);
		for i in 0..lens { fa[i] *= fb[i]; }
		fft_opt(&mut fa, true);
		fa.iter().take(a.len() + b.len() - 1).copied().collect()
	}
}
pub use ntt::{fft_opt, polymul};
