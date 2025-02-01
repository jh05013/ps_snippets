#[allow(unused_imports)]
use std::collections::*;

fn solve<R: std::io::BufRead, W: std::io::Write>(
	oj: &mut io::OJ<R, W>
) {
	;
}



// https://github.com/jh05013/ps_snippets

fn main() {
	let mut oj = io::stdin();
	
	for tc in 0..oj.usize() {
		oj.write("Case #").write(tc+1).write(": ");
		solve(&mut oj);
	}
}

