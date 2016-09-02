use std::borrow::Cow;
use std::str::CharIndices;
use {Casing, Locale};

impl Casing for str {
	fn upper(&self, _locale: Locale) -> Cow<Self> {
		let mut chars = self.char_indices();

		while let Some((start, ch)) = chars.next() {
			// There's a lower case character, gotta copy the string.
			if !ch.is_uppercase() {
				return Cow::Owned(owned(self, chars, (start, ch)));
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, chars: CharIndices, (start, ch): (usize, char)) -> String {
			let mut result = String::with_capacity(this.len());
			result.push_str(&this[.. start]);
			result.extend(ch.to_uppercase());

			// The already upper case starting offset, if any.
			let mut leftover = None;

			// Try to collect slices of upper case characters to push into the
			// result or extend with the upper case version if a lower case
			// character is found.
			for (i, ch) in chars {
				if !ch.is_uppercase() {
					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
					}

					result.extend(ch.to_uppercase());
				}
				else if leftover.is_none() {
					leftover = Some(i);
				}
			}

			// Append any leftover upper case characters.
			if let Some(offset) = leftover.take() {
				result.push_str(&this[offset ..]);
			}

			result
		}
	}

	fn lower(&self, _locale: Locale) -> Cow<Self> {
		let mut chars = self.char_indices();

		while let Some((start, ch)) = chars.next() {
			// There's an upper case character, gotta copy the string.
			if !ch.is_lowercase() {
				return Cow::Owned(owned(self, chars, (start, ch)));
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, chars: CharIndices, (start, ch): (usize, char)) -> String {
			let mut result = String::with_capacity(this.len());
			result.push_str(&this[.. start]);
			result.extend(ch.to_lowercase());

			// The already lower case starting offset, if any.
			let mut leftover = None;

			// Try to collect slices of upper case characters to push into the
			// result or extend with the upper case version if a lower case
			// character is found.
			for (i, ch) in chars {
				if !ch.is_lowercase() {
					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
					}

					result.extend(ch.to_lowercase());
				}
				else if leftover.is_none() {
					leftover = Some(i);
				}
			}

			// Append any leftover lower case characters.
			if let Some(offset) = leftover.take() {
				result.push_str(&this[offset ..]);
			}

			result
		}
	}

	fn capitalized(&self, _locale: Locale) -> Cow<Self> {
		if let Some(first) = self.chars().next() {
			// If the first letter is already uppercase we don't need to do anything.
			if !first.is_lowercase() {
				return Cow::Borrowed(self);
			}

			return Cow::Owned(owned(self, first));
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, first: char) -> String {
			let mut result = String::with_capacity(this.len());
			result.extend(first.to_uppercase());

			if let Some((i, _)) = this.char_indices().skip(1).next() {
				result.push_str(&this[i..]);
			}

			result
		}
	}

	fn camel(&self, mode: super::Camel, _locale: Locale) -> Cow<Self> {
		let mut chars    = self.char_indices();
		let mut new_word = mode == super::Camel::Upper;

		while let Some((start, ch)) = chars.next() {
			if (new_word && !ch.is_uppercase()) || (!new_word && ch.is_uppercase()) || ch == '-' || ch == '_' {
				return Cow::Owned(owned(self, chars, (start, ch), new_word));
			}
			else {
				new_word = false;
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, chars: CharIndices, (start, ch): (usize, char), mut new_word: bool) -> String {
			let mut result = String::with_capacity(this.len());
			result.push_str(&this[.. start]);

			if ch != '-' && ch != '_' {
				if new_word {
					result.extend(ch.to_uppercase());
				}
				else {
					result.push(ch);
				}
			}

			// The already properly cased starting offset, if any.
			let mut leftover = None;
			        new_word = ch == '-' || ch == '_';

			for (i, ch) in chars {
				if new_word && !ch.is_uppercase() {
					new_word = false;

					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
					}

					result.extend(ch.to_uppercase());
				}
				else if !new_word && ch.is_uppercase() {
					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
					}

					result.extend(ch.to_lowercase());
				}
				else if ch == '-' || ch == '_' {
					new_word = true;

					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
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
				result.push_str(&this[offset ..]);
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
		let mut chars    = self.char_indices();
		let mut new_word = true;

		while let Some((start, ch)) = chars.next() {
			if new_word && !ch.is_uppercase() {
				return Cow::Owned(owned(self, chars, (start, ch)));
			}
			else if ch == '-' {
				new_word = true;
			}
			else {
				new_word = false;
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, chars: CharIndices, (start, ch): (usize, char)) -> String {
			let mut result = String::with_capacity(this.len());
			result.push_str(&this[.. start]);
			result.extend(ch.to_uppercase());

			// The already properly cased starting offset, if any.
			let mut leftover = None;
			let mut new_word = false;

			for (i, ch) in chars {
				if new_word && !ch.is_uppercase() {
					new_word = false;

					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
					}

					result.extend(ch.to_uppercase());
				}
				else {
					if ch == '-' {
						new_word = true;
					}

					if leftover.is_none() {
						leftover = Some(i);
					}
				}
			}

			// Append any leftover upper case characters.
			if let Some(offset) = leftover.take() {
				result.push_str(&this[offset ..]);
			}

			result
		}
	}
}

#[cfg(test)]
mod test {
	use std::borrow::Cow;
	use {Casing, Camel};

	#[test]
	fn upper_borrow() {
		assert_eq!("FOO".upper(Default::default()), Cow::Borrowed("FOO"));
	}

	#[test]
	fn upper_owned() {
		assert_eq!("FoO".upper(Default::default()), Cow::Owned::<str>("FOO".into()));
		assert_eq!("fßoß".upper(Default::default()), Cow::Owned::<str>("FSSOSS".into()));
		assert_eq!("fßoßo".upper(Default::default()), Cow::Owned::<str>("FSSOSSO".into()));
		assert_eq!("fßoßoooooo".upper(Default::default()), Cow::Owned::<str>("FSSOSSOOOOOO".into()));
	}

	#[test]
	fn lower_borrow() {
		assert_eq!("foo".lower(Default::default()), Cow::Borrowed("foo"));
		assert_eq!("ßðđ".lower(Default::default()), Cow::Borrowed("ßðđ"));
	}

	#[test]
	fn lower_owned() {
		assert_eq!("FoO".lower(Default::default()), Cow::Owned::<str>("foo".into()));
		assert_eq!("fSSoSS".lower(Default::default()), Cow::Owned::<str>("fssoss".into()));
	}

	#[test]
	fn capitalized_borrow() {
		assert_eq!("Foo".capitalized(Default::default()), Cow::Borrowed("Foo"));
		assert_eq!("FoO".capitalized(Default::default()), Cow::Borrowed("FoO"));
	}

	#[test]
	fn capitalized_owned() {
		assert_eq!("foo".capitalized(Default::default()), Cow::Owned::<str>("Foo".into()));
		assert_eq!("foO".capitalized(Default::default()), Cow::Owned::<str>("FoO".into()));
	}

	#[test]
	fn camel_borrow() {
		assert_eq!("FooBar".camel(Camel::Upper, Default::default()), Cow::Borrowed("FooBar"));
		assert_eq!("Foo".camel(Camel::Upper, Default::default()), Cow::Borrowed("Foo"));

		assert_eq!("fooBar".camel(Camel::Lower, Default::default()), Cow::Borrowed("fooBar"));
		assert_eq!("foo".camel(Camel::Lower, Default::default()), Cow::Borrowed("foo"));
	}

	#[test]
	fn camel_owned() {
		assert_eq!("FooBar", "Foo-bar".camel(Camel::Upper, Default::default()));
		assert_eq!("FooBar", "foo_bar".camel(Camel::Upper, Default::default()));

		assert_eq!("fooBar", "foo-Bar".camel(Camel::Lower, Default::default()));
		assert_eq!("fooBar", "foo_bar".camel(Camel::Lower, Default::default()));
	}

	#[test]
	fn header_borrow() {
		assert_eq!("Foo".header(Default::default()), Cow::Borrowed("Foo"));
		assert_eq!("Foo-Bar".header(Default::default()), Cow::Borrowed("Foo-Bar"));
		assert_eq!("Foo-Bar-Baz".header(Default::default()), Cow::Borrowed("Foo-Bar-Baz"));
		assert_eq!("MIME-Type".header(Default::default()), Cow::Borrowed("MIME-Type"));
	}

	#[test]
	fn header_owned() {
		assert_eq!("foo".header(Default::default()), Cow::Owned::<str>("Foo".into()));
		assert_eq!("foo-bar".header(Default::default()), Cow::Owned::<str>("Foo-Bar".into()));
		assert_eq!("foo-bar-baz".header(Default::default()), Cow::Owned::<str>("Foo-Bar-Baz".into()));
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
		b.iter(|| "Foo-Bar-Baz".header(Default::default()))
	}

	#[bench]
	fn camel_borrowed(b: &mut Bencher) {
		b.iter(|| "FooBarBaz".header(Default::default()))
	}

	#[bench]
	fn header_owned(b: &mut Bencher) {
		b.iter(|| "Foo-Bar-baz".header(Default::default()))
	}

	#[bench]
	fn header_borrowed(b: &mut Bencher) {
		b.iter(|| "Foo-Bar-Baz".header(Default::default()))
	}
}
