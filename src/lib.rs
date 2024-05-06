//! This is a collection of Rust algorithm snippets maintained by
//! jh05013. This is mainly aimed for competitive programming and
//! problem solving.
//! 
//! # Disclaimer
//! While I tried to keep some degree of readability and good API,
//! this crate is made primarily for online judges.
//! This puts the following restrictions:
//! - No external dependencies.
//! - Can't use relatively new features, since some online judges
//!   don't have up-to-date compilers.
//! - Must be easy to copy-paste into the main file, and then
//!   said main file should still be easy to navigate, since most
//!   online judges don't support multiple files.
//! 
//! As such, there are some styling choices different from most crates:
//! - Tabs instead of spaces to save bytes.
//! - Each module is stored in a separate file (easy to copy-paste),
//!   and wrapped with one `mod` (easy to navigate). The mod is then
//!   immediately re-exported so that users don't have to type out the
//!   mod names.
//! - Very short blocks are written in one line to save bytes, e.g.
//!   `if a == b { return None; }`
//! 
//! Furthermore, the API can change at any time and there will be no versioning.
//! Do NOT use this as an external crate.
//! 
//! # Supported Versions
//! Since most online judges don't update their compilers frequently,
//! it is important to support much older versions of Rust.
//! However, we can't go all the way back to like 1.0 since that would
//! limit the implementation too much.
//! 
//! The MSRV for this crate is **1.56**.
//! The table of Rust versions (last updated 2024 May 5) is as follows:
//! 
//! OJ | Version
//! -- | --
//! [Kattis](https://icpc.kattis.com/) | [1.77](https://icpc.kattis.com/languages/rust)
//! [DMOJ](https://dmoj.ca/) | [1.76](https://dmoj.ca/status/)
//! [Codeforces](https://codeforces.com/) | [1.75](https://codeforces.com/problemset/customtest)
//! [Timus](https://acm.timus.ru/) | [1.75](https://acm.timus.ru/submit.aspx)
//! [Leetcode](https://leetcode.com/problemset/) | [1.74](https://support.leetcode.com/hc/en-us/articles/360011833974-What-are-the-environments-for-the-programming-languages)
//! [Baekjoon](https://acmicpc.net/) | [1.73](https://help.acmicpc.net/language/info)
//! [Library Checker](https://judge.yosupo.jp/) | [1.71](https://judge.yosupo.jp/help)
//! [CSAcademy](https://csacademy.com/) | [1.71](https://csacademy.com/about/environment/)
//! [Atcoder](https://atcoder.jp/home) | [1.70](https://atcoder.jp/contests/practice2/custom_test)
//! [QOJ](https://qoj.ac/) | [1.70](https://qoj.ac/blog/qingyu/archive)
//! [HackerEarth](https://www.hackerearth.com/challenges/) | [1.68](https://help.hackerearth.com/hc/en-us/articles/360002371373-supported-browsers-and-languages)
//! [HackerRank](https://www.hackerrank.com/dashboard) | [1.59](https://candidatesupport.hackerrank.com/hc/en-us/articles/4402913877523-Execution-Environment-and-Samples)
//! [toph.co](https://toph.co/) | [1.57](https://toph.co/languages)
//! [Sphere](https://www.spoj.com/) | [1.56](https://www.spoj.com/submit/TEST/)
//! [eolymp](https://basecamp.eolymp.com/en) | ❌[1.46](https://basecamp.eolymp.com/en/problems/1)
//! [Aizu](https://onlinejudge.u-aizu.ac.jp/home) | ❌[1.41](https://onlinejudge.u-aizu.ac.jp/system_info)
//! 
//! Can't find information: [TLX](https://tlx.toki.id/)
//! 
//! CodeChef has absolutely bloated and terrible UI, don't even bother.
//! 
//! # Other Rust Snippets
//! - [bamgoeSN](https://bamgoesn.github.io/rust-ps-md/intro.html)
//! - [kiwiyou](https://snippets.kiwiyou.dev/)
//! - [Bubbler](https://github.com/Bubbler-4/rust-problem-solving/tree/main/competitive/src/competelib)

pub mod io;

pub mod ds;
pub mod geometry;
pub mod graph;
pub mod math;
pub mod matrix;
pub mod number;
pub mod prime;
pub mod rand;
pub mod string;

pub mod slow;

mod test;
