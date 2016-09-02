use std::borrow::Cow;
use std::iter::{Cloned, Enumerate};
use std::ascii::AsciiExt;
use std::slice;
use {Casing, Locale};

#[inline(always)]
fn is_ascii_uppercase(b: u8) -> bool {
	b >= b'A' && b <= b'Z'
}

#[inline(always)]
fn is_ascii_lowercase(b: u8) -> bool {
	!is_ascii_uppercase(b)
}

impl Casing for [u8] {
	fn upper(&self, _locale: Locale) -> Cow<Self> {
		let mut chars = self.iter().cloned().enumerate();

		while let Some((start, ch)) = chars.next() {
			// There's a lower case character, gotta copy the string.
			if !is_ascii_uppercase(ch) {
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
				if !is_ascii_uppercase(ch) {
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
			if !is_ascii_lowercase(ch) {
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
				if !is_ascii_lowercase(ch) {
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
		if let Some(&first) = self.get(0) {
			// If the first letter is already uppercase we don't need to do anything.
			if !is_ascii_lowercase(first) {
				return Cow::Borrowed(self);
			}

			return Cow::Owned(owned(self, first));
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], first: u8) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.push(first.to_ascii_uppercase());
			result.extend_from_slice(&this[1..]);

			result
		}
	}

	fn camel(&self, mode: super::Camel, _locale: Locale) -> Cow<Self> {
		let mut chars    = self.iter().cloned().enumerate();
		let mut new_word = mode == super::Camel::Upper;

		while let Some((start, ch)) = chars.next() {
			if (new_word && !is_ascii_uppercase(ch)) || (!new_word && is_ascii_uppercase(ch)) || ch == b'-' || ch == b'_' {
				return Cow::Owned(owned(self, chars, (start, ch), new_word));
			}
			else {
				new_word = false;
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &[u8], chars: Enumerate<Cloned<slice::Iter<u8>>>, (start, ch): (usize, u8), mut new_word: bool) -> Vec<u8> {
			let mut result = Vec::with_capacity(this.len());
			result.extend_from_slice(&this[.. start]);

			if ch != b'-' && ch != b'_' {
				if new_word {
					result.push(ch.to_ascii_uppercase());
				}
				else {
					result.push(ch);
				}
			}

			// The already properly cased starting offset, if any.
			let mut leftover = None;
			        new_word = ch == b'-' || ch == b'_';

			for (i, ch) in chars {
				if new_word && !is_ascii_uppercase(ch) {
					new_word = false;

					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(ch.to_ascii_uppercase());
				}
				else if !new_word && is_ascii_uppercase(ch) {
					if let Some(offset) = leftover.take() {
						result.extend_from_slice(&this[offset .. i]);
					}

					result.push(ch.to_ascii_lowercase());
				}
				else if ch == b'-' || ch == b'_' {
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

	fn snake(&self, _locale: Locale) -> Cow<Self> {
		Cow::Borrowed(self)
	}

	fn dashed(&self, _locale: Locale) -> Cow<Self> {
		Cow::Borrowed(self)
	}

	fn header(&self, _locale: Locale) -> Cow<Self> {
		let mut chars    = self.iter().cloned().enumerate();
		let mut new_word = true;

		while let Some((start, ch)) = chars.next() {
			if new_word && !is_ascii_uppercase(ch) {
				let mut result = Vec::with_capacity(self.len());
				result.extend_from_slice(&self[.. start]);
				result.push(ch.to_ascii_uppercase());

				// The already properly cased starting offset, if any.
				let mut leftover = None;
				        new_word = false;

				while let Some((i, ch)) = chars.next() {
					if new_word && !is_ascii_uppercase(ch) {
						new_word = false;

						if let Some(offset) = leftover.take() {
							result.extend_from_slice(&self[offset .. i]);
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
					result.extend_from_slice(&self[offset ..]);
				}

				return Cow::Owned(result);
			}
			else if ch == b'-' {
				new_word = true;
			}
			else {
				new_word = false;
			}
		}

		Cow::Borrowed(self)
	}
}

#[cfg(test)]
mod test {
	use std::borrow::Cow;
	use {Casing, Camel};

	#[test]
	fn upper_borrow() {
		assert_eq!(b"FOO".upper(Default::default()), Cow::Borrowed(b"FOO"));
	}

	#[test]
	fn upper_owned() {
		assert_eq!(b"FoO".upper(Default::default()).into_owned(), b"FOO".to_vec());
		assert_eq!("fßoß".as_bytes().upper(Default::default()).into_owned(), "FßOß".as_bytes().to_vec());
		assert_eq!("fßoßo".as_bytes().upper(Default::default()).into_owned(), "FßOßO".as_bytes().to_vec());
		assert_eq!("fßoßoooooo".as_bytes().upper(Default::default()).into_owned(), "FßOßOOOOOO".as_bytes().to_vec());
	}

	#[test]
	fn lower_borrow() {
		assert_eq!(b"foo".lower(Default::default()), Cow::Borrowed(b"foo"));
		assert_eq!("ßðđ".as_bytes().lower(Default::default()), Cow::Borrowed("ßðđ".as_bytes()));
	}

	#[test]
	fn lower_owned() {
		assert_eq!(b"FoO".lower(Default::default()).into_owned(), b"foo".to_vec());
		assert_eq!(b"fSSoSS".lower(Default::default()).into_owned(), b"fssoss".to_vec());
	}

	#[test]
	fn capitalized_borrow() {
		assert_eq!(b"Foo".to_vec(), b"Foo".capitalized(Default::default()).into_owned());
		assert_eq!(b"FoO".to_vec(), b"FoO".capitalized(Default::default()).into_owned());
	}

	#[test]
	fn capitalized_owned() {
		assert_eq!(b"foo".capitalized(Default::default()).into_owned(), b"Foo".to_vec());
		assert_eq!(b"foO".capitalized(Default::default()).into_owned(), b"FoO".to_vec());
	}

	#[test]
	fn camel_borrow() {
		assert_eq!(b"FooBar".camel(Camel::Upper, Default::default()).into_owned(), b"FooBar".to_vec());
		assert_eq!(b"Foo".camel(Camel::Upper, Default::default()).into_owned(), b"Foo".to_vec());

		assert_eq!(b"fooBar".to_vec(), b"fooBar".camel(Camel::Lower, Default::default()).into_owned());
		assert_eq!(b"foo".to_vec(), b"foo".camel(Camel::Lower, Default::default()).into_owned());
	}

	#[test]
	fn camel_owned() {
		assert_eq!(b"FooBar".to_vec(), b"Foo-bar".camel(Camel::Upper, Default::default()).into_owned());
		assert_eq!(b"FooBar".to_vec(), b"foo_bar".camel(Camel::Upper, Default::default()).into_owned());

		assert_eq!(b"fooBar".to_vec(), b"foo-Bar".camel(Camel::Lower, Default::default()).into_owned());
		assert_eq!(b"fooBar".to_vec(), b"foo_bar".camel(Camel::Lower, Default::default()).into_owned());
	}

	#[test]
	fn header_borrow() {
		assert_eq!(b"Foo".header(Default::default()), Cow::Borrowed(b"Foo"));
		assert_eq!(b"Foo-Bar".header(Default::default()), Cow::Borrowed(b"Foo-Bar"));
		assert_eq!(b"Foo-Bar-Baz".header(Default::default()), Cow::Borrowed(b"Foo-Bar-Baz"));
		assert_eq!(b"MIME-Type".header(Default::default()), Cow::Borrowed(b"MIME-Type"));
	}

	#[test]
	fn header_owned() {
		assert_eq!(b"foo".header(Default::default()).into_owned(), b"Foo".to_vec());
		assert_eq!(b"foo-bar".header(Default::default()).into_owned(), b"Foo-Bar".to_vec());
		assert_eq!(b"foo-bar-baz".header(Default::default()).into_owned(), b"Foo-Bar-Baz".to_vec());
	}
}

#[cfg(test)]
mod bench {
	use test::Bencher;
	use Casing;

	#[bench]
	fn upper_owned_early(b: &mut Bencher) {
		b.iter(|| "AAAAAaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAa".upper(Default::default()));
	}

	#[bench]
	fn upper_owned_late(b: &mut Bencher) {
		b.iter(|| "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAa".upper(Default::default()));
	}

	#[bench]
	fn upper_owned_all(b: &mut Bencher) {
		b.iter(|| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".upper(Default::default()));
	}

	#[bench]
	fn upper_owned_std(b: &mut Bencher) {
		b.iter(|| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_uppercase());
	}

	#[bench]
	fn upper_borrow_short(b: &mut Bencher) {
		b.iter(|| "AAAAAAA".upper(Default::default()));
	}

	#[bench]
	fn upper_borrow_long(b: &mut Bencher) {
		b.iter(|| "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".upper(Default::default()));
	}

	#[bench]
	fn lower_owned_early(b: &mut Bencher) {
		b.iter(|| "aaaaaAaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaA".lower(Default::default()));
	}

	#[bench]
	fn lower_owned_all(b: &mut Bencher) {
		b.iter(|| "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".lower(Default::default()));
	}

	#[bench]
	fn lower_owned_std(b: &mut Bencher) {
		b.iter(|| "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_lowercase());
	}

	#[bench]
	fn lower_owned_late(b: &mut Bencher) {
		b.iter(|| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaA".lower(Default::default()));
	}

	#[bench]
	fn lower_borrow_short(b: &mut Bencher) {
		b.iter(|| "aaaaaaa".lower(Default::default()));
	}

	#[bench]
	fn lower_borrow_long(b: &mut Bencher) {
		b.iter(|| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".lower(Default::default()));
	}

	#[bench]
	fn capitalized_owned(b: &mut Bencher) {
		b.iter(|| "aAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".capitalized(Default::default()));
	}

	#[bench]
	fn capitalized_borrowed(b: &mut Bencher) {
		b.iter(|| "Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".capitalized(Default::default()));
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
