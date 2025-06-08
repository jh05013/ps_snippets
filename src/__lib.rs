//! This is a collection of Rust algorithm snippets maintained by
//! jh05013. This is mainly aimed for competitive programming and
//! problem solving.
//! 
//! # How to Use This Repository
//! 
//! Clone it. Make a `src/main.rs` file and write your code
//! there. To generate a submission for OJ, use
//! [cargo-equip](https://github.com/qryxip/cargo-equip).
//! 
//! If you use [Nushell](https://www.nushell.sh/), you can
//! use `equip.nu` to generate submissions quickly. For maximum
//! efficiency, add `alias e = nu equip.nu` at `config.nu`.
//! 
//! # Supported Versions
//! Since some online judges have rather old compilers,
//! it is important to support much older versions of Rust.
//! However, we can't go all the way back to like 1.0 since that would
//! limit the implementation too much.
//! 
//! The MSRV for this crate is **1.61**, mainly because
//! `oj` requires [stdio handles to have static lifetimes](https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html#static-handles-for-locked-stdio).
//! The table of Rust versions (last updated 2025 Jun 1)
//! is as follows:
//! 
//! OJ | Version
//! -- | --
//! [Kattis](https://icpc.kattis.com/) | [1.85](https://icpc.kattis.com/languages/rust)
//! [Leetcode](https://leetcode.com/problemset/) | [1.85](https://support.leetcode.com/hc/en-us/articles/360011833974-What-are-the-environments-for-the-programming-languages)
//! [Toph](https://toph.co/) | [1.84](https://toph.co/languages)
//! [QOJ](https://qoj.ac/) | [1.84](https://qoj.ac/blog/qingyu/blog/221)
//! [DMOJ](https://dmoj.ca/) | [1.83](https://dmoj.ca/status/)
//! [Aizu](https://onlinejudge.u-aizu.ac.jp/home) | [1.79](https://onlinejudge.u-aizu.ac.jp/system_info)
//! [eolymp](https://basecamp.eolymp.com/en) | [1.78](https://basecamp.eolymp.com/en/problems/1)
//! [Codeforces](https://codeforces.com/) | [1.75](https://codeforces.com/problemset/customtest)
//! [Timus](https://acm.timus.ru/) | [1.75](https://acm.timus.ru/submit.aspx)
//! [Baekjoon](https://acmicpc.net/) | [1.73](https://help.acmicpc.net/language/info)
//! [Library Checker](https://judge.yosupo.jp/) | [1.71](https://judge.yosupo.jp/help)
//! [CSAcademy](https://csacademy.com/) | [1.71](https://csacademy.com/about/environment/)
//! [Atcoder](https://atcoder.jp/home) | [1.70](https://atcoder.jp/contests/practice2/custom_test)
//! [HackerEarth](https://www.hackerearth.com/challenges/) | [1.68](https://help.hackerearth.com/hc/en-us/articles/360002371373-supported-browsers-and-languages)
//! [HackerRank](https://www.hackerrank.com/dashboard) | ❌[1.59](https://candidatesupport.hackerrank.com/hc/en-us/articles/4402913877523-Execution-Environment)
//! [Sphere](https://www.spoj.com/) | ❌[1.56](https://www.spoj.com/submit/TEST/)
//! 
//! Can't find information: [TLX](https://tlx.toki.id/)
//! 
//! # Other Rust Snippets
//! - [bamgoeSN](https://bamgoesn.github.io/rust-ps-md/intro.html)
//! - [kiwiyou](https://snippets.kiwiyou.dev/)
//! - [Bubbler](https://github.com/Bubbler-4/rust-problem-solving/tree/main/competitive/src/competelib)
