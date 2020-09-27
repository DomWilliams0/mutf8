use mutf8::utf8_to_mutf8;
use mutf8::{mutf8_to_utf8, StrExt};

macro_rules! assert_owned {
	($var:ident, $msg:expr) => {{
		use std::borrow::Cow;
		if let Cow::Borrowed(_) = $var {
			panic!($msg)
			}
		}};
}

macro_rules! assert_borrowed {
	($var:ident, $msg:expr) => {{
		use std::borrow::Cow;
		if let Cow::Owned(_) = $var {
			panic!($msg)
			}
		}};
}

#[test]
fn ascii_test() {
	let data = utf8_to_mutf8(b"value");
	assert_eq!(data.len(), 5);

	assert_borrowed!(
		data,
		"Data not borrowed. [It's just pure ascii which uses the same encoding as utf8]"
	);
}

#[test]
fn nul_test() {
	let data = utf8_to_mutf8(b"\0");
	assert_eq!(data.len(), 2);

	assert_owned!(
		data,
		"Data not owned. [A nul byte needs two bytes in mutf8]"
	);
}

#[test]
fn mutf8_literal() {
	// wont panic
	"nice".as_mstr();
}

#[test]
#[should_panic]
fn mutf8_literal_bad() {
	// will panic
	"cool\0nice".as_mstr();
}
