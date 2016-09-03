use std::borrow::Cow;
use std::iter::{Cloned, Enumerate};
use std::ascii::AsciiExt;
use std::slice;
use {Casing, Separator, Locale};

#[inline(always)]
fn is_ascii_uppercase(b: u8) -> bool {
	b >= b'A' && b <= b'Z'
}

#[inline(always)]
fn is_ascii_lowercase(b: u8) -> bool {
	b >= b'a' && b <= b'z'
}

#[inline(always)]
fn is_ascii_alphabetic(b: u8) -> bool {
	(b >= b'A' && b <= b'Z') || (b >= b'a' && b <= b'z')
}

impl Casing for [u8] {
	type Character = u8;

	fn upper(&self, _locale: Locale) -> Cow<Self> {
		let mut chars = self.iter().cloned().enumerate();

		while let Some((start, ch)) = chars.next() {
			// There's a lower case character, gotta copy the string.
			if !is_ascii_uppercase(ch) && is_ascii_alphabetic(ch) {
				return Cow::Owned(owned(self, chars, (start, ch)));
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], chars: Enumerate<Cloned<slice::Iter<u8>>>, (start, ch): (usize, u8)) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.extend_from_slice(&this[.. start]);
			result.push(ch.to_ascii_uppercase());

			// The already upper case starting offset, if any.
			let mut leftover = None;

			// Try to collect slices of upper case characters to push into the
			// result or extend with the upper case version if a lower case
			// character is found.
			for (i, ch) in chars {
				if !is_ascii_uppercase(ch) && is_ascii_alphabetic(ch) {
					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(ch.to_ascii_uppercase());
				}
				else if leftover.is_none() {
					leftover = Some(i);
				}
			}

			// Append any leftover upper case characters.
			if let Some(offset) = leftover.take() {
				result.extend_from_slice(&this[offset ..]);
			}

			result
		}
	}

	fn lower(&self, _locale: Locale) -> Cow<Self> {
		let mut chars = self.iter().cloned().enumerate();

		while let Some((start, ch)) = chars.next() {
			// There's a lower case character, gotta copy the string.
			if !is_ascii_lowercase(ch) && is_ascii_alphabetic(ch) {
				return Cow::Owned(owned(self, chars, (start, ch)));
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], chars: Enumerate<Cloned<slice::Iter<u8>>>, (start, ch): (usize, u8)) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.extend_from_slice(&this[.. start]);
			result.push(ch.to_ascii_lowercase());

			// The already upper case starting offset, if any.
			let mut leftover = None;

			// Try to collect slices of upper case characters to push into the
			// result or extend with the upper case version if a lower case
			// character is found.
			for (i, ch) in chars {
				if !is_ascii_lowercase(ch) && is_ascii_alphabetic(ch) {
					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(ch.to_ascii_lowercase());
				}
				else if leftover.is_none() {
					leftover = Some(i);
				}
			}

			// Append any leftover upper case characters.
			if let Some(offset) = leftover.take() {
				result.extend_from_slice(&this[offset ..]);
			}

			result
		}
	}

	fn capitalized(&self, _locale: Locale) -> Cow<Self> {
		let mut chars = self.iter().cloned().enumerate();

		if let Some((start, ch)) = chars.next() {
			// If the first letter is already uppercase we don't need to do anything.
			if !is_ascii_uppercase(ch) && is_ascii_alphabetic(ch) {
				return Cow::Owned(owned(self, chars, (start, ch), true));
			}

			while let Some((start, ch)) = chars.next() {
				// There's an upper case character, gotta copy the string.
				if !is_ascii_lowercase(ch) && is_ascii_alphabetic(ch) {
					return Cow::Owned(owned(self, chars, (start, ch), false));
				}
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], chars: Enumerate<Cloned<slice::Iter<u8>>>, (start, ch): (usize, u8), upcase: bool) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.extend_from_slice(&this[.. start]);

			if upcase {
				result.push(ch.to_ascii_uppercase());
			}
			else {
				result.push(ch.to_ascii_lowercase());
			}

			// The already lower case starting offset, if any.
			let mut leftover = None;

			// Try to collect slices of upper case characters to push into the
			// result or extend with the upper case version if a lower case
			// character is found.
			for (i, ch) in chars {
				if !is_ascii_lowercase(ch) && is_ascii_alphabetic(ch) {
					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(ch.to_ascii_lowercase());
				}
				else if leftover.is_none() {
					leftover = Some(i);
				}
			}

			// Append any leftover lower case characters.
			if let Some(offset) = leftover.take() {
				result.extend_from_slice(&this[offset ..]);
			}

			result
		}
	}

	fn camel(&self, separator: Separator<&[u8]>, mode: super::Camel, _locale: Locale) -> Cow<Self> {
		let mut chars    = self.iter().cloned().enumerate();
		let mut new_word = mode == super::Camel::Upper;

		while let Some((start, ch)) = chars.next() {
			if new_word && !is_ascii_uppercase(ch) && is_ascii_alphabetic(ch) {
				return Cow::Owned(owned(self, separator, chars, (start, ch), new_word));
			}
			else if separator.iter().any(|&c| ch == c) {
				return Cow::Owned(owned(self, separator, chars, (start, ch), true));
			}
			else {
				new_word = false;
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], separator: Separator<&[u8]>, chars: Enumerate<Cloned<slice::Iter<u8>>>, (start, ch): (usize, u8), mut new_word: bool) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.extend_from_slice(&this[.. start]);

			if separator.iter().all(|&c| ch != c) {
				if new_word {
					result.push(ch.to_ascii_uppercase());
				}
				else {
					result.push(ch);
				}
			}

			// The already properly cased starting offset, if any.
			let mut leftover = None;
			        new_word = separator.iter().any(|&c| ch == c);

			for (i, ch) in chars {
				if new_word && !is_ascii_uppercase(ch) && is_ascii_alphabetic(ch) {
					new_word = false;

					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(ch.to_ascii_uppercase());
				}
				else if separator.iter().any(|&c| ch == c) {
					new_word = true;

					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}
				}
				else {
					if leftover.is_none() {
						leftover = Some(i);
					}

					new_word = false;
				}
			}

			// Append any leftover upper case characters.
			if let Some(offset) = leftover.take() {
				result.extend_from_slice(&this[offset ..]);
			}

			result
		}
	}

	fn separated(&self, separator: Separator<u8>, _locale: Locale) -> Cow<Self> {
		let mut chars = self.iter().cloned().enumerate();

		while let Some((start, ch)) = chars.next() {
			if ch != separator.0 && !is_ascii_lowercase(ch) {
				return Cow::Owned(owned(self, separator, chars, (start, ch)));
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], separator: Separator<u8>, chars: Enumerate<Cloned<slice::Iter<u8>>>, (start, ch): (usize, u8)) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.extend_from_slice(&this[.. start]);
			result.push(separator.0);

			if is_ascii_alphabetic(ch) {
				result.push(ch.to_ascii_lowercase());
			}

			// The already lower case starting offset, if any.
			let mut leftover = None;

			for (i, ch) in chars {
				if ch != separator.0 && !is_ascii_lowercase(ch) {
					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(separator.0);

					if is_ascii_alphabetic(ch) {
						result.push(ch.to_ascii_lowercase());
					}
				}
				else if leftover.is_none() {
					leftover = Some(i);
				}
			}

			// Append any leftover lower case characters.
			if let Some(offset) = leftover.take() {
				result.extend_from_slice(&this[offset ..]);
			}

			result
		}
	}

	fn header(&self, _locale: Locale) -> Cow<Self> {
		let mut chars    = self.iter().cloned().enumerate();
		let mut new_word = true;

		while let Some((start, ch)) = chars.next() {
			if new_word && !is_ascii_uppercase(ch) {
				return Cow::Owned(owned(self, chars, (start, ch)));
			}
			else if ch == b'-' {
				new_word = true;
			}
			else {
				new_word = false;
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], chars: Enumerate<Cloned<slice::Iter<u8>>>, (start, ch): (usize, u8)) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.extend_from_slice(&this[.. start]);
			result.push(ch.to_ascii_uppercase());

			// The already properly cased starting offset, if any.
			let mut leftover = None;
			let mut new_word = false;

			for (i, ch) in chars {
				if new_word && !is_ascii_uppercase(ch) {
					new_word = false;

					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(ch.to_ascii_uppercase());
				}
				else {
					if ch == b'-' {
						new_word = true;
					}

					if leftover.is_none() {
						leftover = Some(i);
					}
				}
			}

			// Append any leftover upper case characters.
			if let Some(offset) = leftover.take() {
				result.extend_from_slice(&this[offset ..]);
			}

			result
		}
	}
}

#[cfg(test)]
mod test {
	use std::borrow::Cow;
	use {Casing, Camel, Separator};

	macro_rules! assert_owned {
		($body:expr) => (
			assert!(match $body {
				Cow::Borrowed(..) => false,
				Cow::Owned(..)    => true,
			})
		);
	}

	macro_rules! assert_borrowed {
		($body:expr) => (
			assert!(match $body {
				Cow::Borrowed(..) => true,
				Cow::Owned(..)    => false,
			})
		);
	}

	#[test]
	fn upper() {
		assert_eq!(b"FOO".to_vec(), b"FOO".upper(Default::default()).into_owned());
		assert_eq!(b"FOO".to_vec(), b"FoO".upper(Default::default()).into_owned());
		assert_eq!("FßOß".as_bytes().to_vec(), "fßoß".as_bytes().upper(Default::default()).into_owned());
		assert_eq!("FßOßO".as_bytes().to_vec(), "fßoßo".as_bytes().upper(Default::default()).into_owned());
		assert_eq!("FßOßOOOOOO".as_bytes().to_vec(), "fßoßoooooo".as_bytes().upper(Default::default()).into_owned());
	}

	#[test]
	fn upper_allocation() {
		assert_borrowed!(b"FOO".upper(Default::default()));
		assert_borrowed!(b"FOO-FOO".upper(Default::default()));
		assert_borrowed!("FOO-ÐÐÐ".as_bytes().upper(Default::default()));
		assert_borrowed!("😀😬😁😂😃😄😅😆😇😉😊🙂🙃☺️😋😌😍😘😗😙😚😜😝😛🤑🤓😎🤗😏😶😐😑😒🙄🤔😳😞😟😠😡😔😕🙁☹️😣😖😫😩😤😮😱😨😰😯😦😧😢😥😪😓😭😵😲🤐😷🤒🤕😴".as_bytes().upper(Default::default()));
		assert_borrowed!("FOO-ßßß".as_bytes().upper(Default::default()));

		assert_owned!(b"Foo".upper(Default::default()));
		assert_owned!(b"FOO-Foo".upper(Default::default()));
	}

	#[test]
	fn lower() {
		assert_eq!(b"foo".to_vec(), b"foo".lower(Default::default()).into_owned());
		assert_eq!("ßðđ".as_bytes().to_vec(), "ßðđ".as_bytes().lower(Default::default()).into_owned());
		assert_eq!(b"foo".to_vec(), b"FoO".lower(Default::default()).into_owned());
		assert_eq!(b"fssoss".to_vec(), b"fSSoSS".lower(Default::default()).into_owned());
	}

	#[test]
	fn lower_allocation() {
		assert_borrowed!("foo".lower(Default::default()));
		assert_borrowed!("foo-foo".lower(Default::default()));
		assert_borrowed!("foo-ßßß".as_bytes().lower(Default::default()));
		assert_borrowed!("😀😬😁😂😃😄😅😆😇😉😊🙂🙃☺️😋😌😍😘😗😙😚😜😝😛🤑🤓😎🤗😏😶😐😑😒🙄🤔😳😞😟😠😡😔😕🙁☹️😣😖😫😩😤😮😱😨😰😯😦😧😢😥😪😓😭😵😲🤐😷🤒🤕😴".as_bytes().upper(Default::default()));

		assert_owned!(b"FOO".lower(Default::default()));
	}

	#[test]
	fn capitalized() {
		assert_eq!(b"Foo".to_vec(), b"Foo".capitalized(Default::default()).into_owned());
		assert_eq!(b"Foo".to_vec(), b"FoO".capitalized(Default::default()).into_owned());
		assert_eq!(b"Foo".to_vec(), b"foo".capitalized(Default::default()).into_owned());
		assert_eq!(b"Foo".to_vec(), b"foO".capitalized(Default::default()).into_owned());
	}

	#[test]
	fn capitalized_allocation() {
		assert_borrowed!(b"Foo".capitalized(Default::default()));
		assert_borrowed!("Foo-foo-æßð".as_bytes().capitalized(Default::default()));

		assert_owned!(b"fOOOOO".capitalized(Default::default()));
		assert_owned!(b"REEEeE".capitalized(Default::default()));
	}

	#[test]
	fn camel() {
		assert_eq!(b"FooBar".to_vec(), b"FooBar".camel(Default::default(), Camel::Upper, Default::default()).into_owned());
		assert_eq!(b"Foo".to_vec(), b"Foo".camel(Default::default(), Camel::Upper, Default::default()).into_owned());
		assert_eq!(b"FooBar".to_vec(), b"Foo-bar".camel(Default::default(), Camel::Upper, Default::default()).into_owned());
		assert_eq!(b"FooBar".to_vec(), b"foo_bar".camel(Default::default(), Camel::Upper, Default::default()).into_owned());

		assert_eq!(b"fooBar".to_vec(), b"foo-Bar".camel(Default::default(), Camel::Lower, Default::default()).into_owned());
		assert_eq!(b"fooBarBaz".to_vec(), b"foo_bar-baz".camel(Default::default(), Camel::Lower, Default::default()).into_owned());
		assert_eq!(b"fooBar".to_vec(), b"fooBar".camel(Default::default(), Camel::Lower, Default::default()).into_owned());
		assert_eq!(b"foo".to_vec(), b"foo".camel(Default::default(), Camel::Lower, Default::default()).into_owned());
	}

	#[test]
	fn camel_allocation() {
		assert_borrowed!(b"FooBar".camel(Default::default(), Camel::Upper, Default::default()));
		assert_borrowed!(b"fooBar".camel(Default::default(), Camel::Lower, Default::default()));
	}

	#[test]
	fn separated() {
		assert_eq!(b"foo_bar".to_vec(), b"foo_bar".separated(Separator(b'_'), Default::default()).into_owned());
		assert_eq!(b"foo-bar-baz".to_vec(), b"foo_bar_baz".separated(Separator(b'-'), Default::default()).into_owned());
	}

	#[test]
	fn separated_allocation() {
		assert_borrowed!(b"foo_bar".separated(Separator(b'_'), Default::default()));
		assert_borrowed!(b"foo-bar".separated(Separator(b'-'), Default::default()));
		assert_borrowed!(b"foo@bar".separated(Separator(b'@'), Default::default()));

		assert_owned!(b"foo_bar".separated(Separator(b'@'), Default::default()));
		assert_owned!(b"foo-bar".separated(Separator(b'_'), Default::default()));
		assert_owned!(b"foo@bar".separated(Separator(b'-'), Default::default()));
	}

	#[test]
	fn header() {
		assert_eq!(b"Foo".to_vec(), b"Foo".header(Default::default()).into_owned());
		assert_eq!(b"Foo-Bar".to_vec(), b"Foo-Bar".header(Default::default()).into_owned());
		assert_eq!(b"Foo-Bar-Baz".to_vec(), b"Foo-Bar-Baz".header(Default::default()).into_owned());
		assert_eq!(b"MIME-Type".to_vec(), b"MIME-Type".header(Default::default()).into_owned());
		assert_eq!(b"Foo".to_vec(), b"foo".header(Default::default()).into_owned());
		assert_eq!(b"Foo-Bar".to_vec(), b"foo-bar".header(Default::default()).into_owned());
		assert_eq!(b"Foo-Bar-Baz".to_vec(), b"foo-bar-baz".header(Default::default()).into_owned());
	}

	#[test]
	fn header_allocation() {
		assert_borrowed!(b"Foo".header(Default::default()));
		assert_borrowed!(b"Foo-Bar".header(Default::default()));
		assert_borrowed!(b"Foo-Bar-Baz".header(Default::default()));
		assert_borrowed!(b"MIME-Type".header(Default::default()));

		assert_owned!(b"foo".header(Default::default()));
		assert_owned!(b"foo-bar".header(Default::default()));
		assert_owned!(b"foo-bar-baz".header(Default::default()));
	}
}

#[cfg(test)]
mod bench {
	use test::Bencher;
	use Casing;

	#[bench]
	fn upper_owned_early(b: &mut Bencher) {
		b.iter(|| b"AAAAAaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAa".upper(Default::default()));
	}

	#[bench]
	fn upper_owned_late(b: &mut Bencher) {
		b.iter(|| b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAa".upper(Default::default()));
	}

	#[bench]
	fn upper_owned_all(b: &mut Bencher) {
		b.iter(|| b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".upper(Default::default()));
	}

	#[bench]
	fn upper_owned_std(b: &mut Bencher) {
		b.iter(|| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_uppercase());
	}

	#[bench]
	fn upper_borrow_short(b: &mut Bencher) {
		b.iter(|| b"AAAAAAA".upper(Default::default()));
	}

	#[bench]
	fn upper_borrow_long(b: &mut Bencher) {
		b.iter(|| b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".upper(Default::default()));
	}

	#[bench]
	fn lower_owned_early(b: &mut Bencher) {
		b.iter(|| b"aaaaaAaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaA".lower(Default::default()));
	}

	#[bench]
	fn lower_owned_all(b: &mut Bencher) {
		b.iter(|| b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".lower(Default::default()));
	}

	#[bench]
	fn lower_owned_std(b: &mut Bencher) {
		b.iter(|| "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_lowercase());
	}

	#[bench]
	fn lower_owned_late(b: &mut Bencher) {
		b.iter(|| b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaA".lower(Default::default()));
	}

	#[bench]
	fn lower_borrow_short(b: &mut Bencher) {
		b.iter(|| b"aaaaaaa".lower(Default::default()));
	}

	#[bench]
	fn lower_borrow_long(b: &mut Bencher) {
		b.iter(|| b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".lower(Default::default()));
	}

	#[bench]
	fn capitalized_owned(b: &mut Bencher) {
		b.iter(|| b"aAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".capitalized(Default::default()));
	}

	#[bench]
	fn capitalized_borrowed(b: &mut Bencher) {
		b.iter(|| b"Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".capitalized(Default::default()));
	}

	#[bench]
	fn camel_owned(b: &mut Bencher) {
		b.iter(|| b"Foo-Bar-Baz".header(Default::default()))
	}

	#[bench]
	fn camel_borrowed(b: &mut Bencher) {
		b.iter(|| b"FooBarBaz".header(Default::default()))
	}

	#[bench]
	fn header_owned(b: &mut Bencher) {
		b.iter(|| b"Foo-Bar-baz".header(Default::default()))
	}

	#[bench]
	fn header_borrowed(b: &mut Bencher) {
		b.iter(|| b"Foo-Bar-Baz".header(Default::default()))
	}
}
