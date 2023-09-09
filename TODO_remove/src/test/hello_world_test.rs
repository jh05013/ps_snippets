#[cfg(test)]

#[test]
fn hello_test() {
	use crate::algo::hello_world::*;
	assert_eq!(hello_world(), "Hello, world!".to_string());
	assert!(hello_world() != "hello, world!".to_string());
}

#[test]
#[should_panic]
fn hello_panic_test() {
	use crate::algo::hello_world::*;
	assert!(hello_world() != "Hello, world!".to_string());
}

#[test]
fn apb_test() {
	use crate::algo::hello_world::*;
	assert_eq!(a_plus_b(1, 2), 3);
}
