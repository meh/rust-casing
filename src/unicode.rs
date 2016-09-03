use std::borrow::Cow;
use std::str::CharIndices;
use {Casing, Locale, Separator};

impl Casing for str {
	type Character = char;

	fn upper(&self, _locale: Locale) -> Cow<Self> {
		let mut chars = self.char_indices();

		while let Some((start, ch)) = chars.next() {
			// There's a lower case character, gotta copy the string.
			if !ch.is_uppercase() && ch.is_alphabetic() {
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
				if !ch.is_uppercase() && ch.is_alphabetic() {
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
			if !ch.is_lowercase() && ch.is_alphabetic() {
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
				if !ch.is_lowercase() && ch.is_alphabetic() {
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
		let mut chars = self.char_indices();

		if let Some((start, ch)) = chars.next() {
			// If the first letter is already uppercase we don't need to do anything.
			if !ch.is_uppercase() && ch.is_alphabetic() {
				return Cow::Owned(owned(self, chars, (start, ch), true));
			}

			while let Some((start, ch)) = chars.next() {
				// There's an upper case character, gotta copy the string.
				if !ch.is_lowercase() && ch.is_alphabetic() {
					return Cow::Owned(owned(self, chars, (start, ch), false));
				}
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, chars: CharIndices, (start, ch): (usize, char), upcase: bool) -> String {
			let mut result = String::with_capacity(this.len());
			result.push_str(&this[.. start]);

			if upcase {
				result.extend(ch.to_uppercase());
			}
			else {
				result.extend(ch.to_lowercase());
			}

			// The already lower case starting offset, if any.
			let mut leftover = None;

			// Try to collect slices of upper case characters to push into the
			// result or extend with the upper case version if a lower case
			// character is found.
			for (i, ch) in chars {
				if !ch.is_lowercase() && ch.is_alphabetic() {
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

	fn camel(&self, separator: Separator<&[char]>, mode: super::Camel, _locale: Locale) -> Cow<Self> {
		let mut chars    = self.char_indices();
		let mut new_word = mode == super::Camel::Upper;

		while let Some((start, ch)) = chars.next() {
			if new_word && !ch.is_uppercase() && ch.is_alphabetic() {
				return Cow::Owned(owned(self, separator, chars, (start, ch), new_word));
			}
			else if separator.0.iter().any(|&c| ch == c) {
				return Cow::Owned(owned(self, separator, chars, (start, ch), true));
			}
			else {
				new_word = false;
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, separator: Separator<&[char]>, chars: CharIndices, (start, ch): (usize, char), mut new_word: bool) -> String {
			let mut result = String::with_capacity(this.len());
			result.push_str(&this[.. start]);

			if separator.0.iter().all(|&c| ch != c) {
				if new_word {
					result.extend(ch.to_uppercase());
				}
				else {
					result.push(ch);
				}
			}

			// The already properly cased starting offset, if any.
			let mut leftover = None;
			        new_word = separator.0.iter().any(|&c| ch == c);

			for (i, ch) in chars {
				if new_word && !ch.is_uppercase() && ch.is_alphabetic() {
					new_word = false;

					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
					}

					result.extend(ch.to_uppercase());
				}
				else if separator.0.iter().any(|&c| ch == c) {
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

	fn separated(&self, separator: Separator<char>, _locale: Locale) -> Cow<Self> {
		let mut chars = self.char_indices();

		while let Some((start, ch)) = chars.next() {
			if ch != separator.0 && !ch.is_lowercase() {
				return Cow::Owned(owned(self, separator, chars, (start, ch)));
			}
		}

		return Cow::Borrowed(self);

		#[inline(always)]
		fn owned(this: &str, separator: Separator<char>, chars: CharIndices, (start, ch): (usize, char)) -> String {
			let mut result = String::with_capacity(this.len());
			result.push_str(&this[.. start]);
			result.push(separator.0);

			if ch.is_alphabetic() {
				result.extend(ch.to_lowercase());
			}

			// The already lower case starting offset, if any.
			let mut leftover = None;

			for (i, ch) in chars {
				if ch != separator.0 && !ch.is_lowercase() {
					if let Some(offset) = leftover.take() {
						result.push_str(&this[offset .. i]);
					}

					result.push(separator.0);

					if ch.is_alphabetic() {
						result.extend(ch.to_lowercase());
					}
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
		assert_eq!("FOO", "FOO".upper(Default::default()));
		assert_eq!("FOO", "FoO".upper(Default::default()));
		assert_eq!("FSSOSS", "fßoß".upper(Default::default()));
		assert_eq!("FSSOSSO", "fßoßo".upper(Default::default()));
		assert_eq!("FSSOSSOOOOOO", "fßoßoooooo".upper(Default::default()));
	}

	#[test]
	fn upper_allocation() {
		assert_borrowed!("FOO".upper(Default::default()));
		assert_borrowed!("FOO-FOO".upper(Default::default()));
		assert_borrowed!("FOO-ÐÐÐ".upper(Default::default()));
		assert_borrowed!("😀😬😁😂😃😄😅😆😇😉😊🙂🙃☺️😋😌😍😘😗😙😚😜😝😛🤑🤓😎🤗😏😶😐😑😒🙄🤔😳😞😟😠😡😔😕🙁☹️😣😖😫😩😤😮😱😨😰😯😦😧😢😥😪😓😭😵😲🤐😷🤒🤕😴".upper(Default::default()));

		assert_owned!("Foo".upper(Default::default()));
		assert_owned!("FOO-Foo".upper(Default::default()));
		assert_owned!("FOO-ßßß".upper(Default::default()));
	}

	#[test]
	fn lower() {
		assert_eq!("foo", "foo".lower(Default::default()));
		assert_eq!("ßðđ", "ßðđ".lower(Default::default()));
		assert_eq!("foo", "FoO".lower(Default::default()));
		assert_eq!("fssoss", "fSSoSS".lower(Default::default()));
	}

	#[test]
	fn lower_allocation() {
		assert_borrowed!("foo".lower(Default::default()));
		assert_borrowed!("foo-foo".lower(Default::default()));
		assert_borrowed!("foo-ßßß".lower(Default::default()));
		assert_borrowed!("😀😬😁😂😃😄😅😆😇😉😊🙂🙃☺️😋😌😍😘😗😙😚😜😝😛🤑🤓😎🤗😏😶😐😑😒🙄🤔😳😞😟😠😡😔😕🙁☹️😣😖😫😩😤😮😱😨😰😯😦😧😢😥😪😓😭😵😲🤐😷🤒🤕😴".upper(Default::default()));

		assert_owned!("FOO".lower(Default::default()));
	}

	#[test]
	fn capitalized() {
		assert_eq!("Foo", "Foo".capitalized(Default::default()));
		assert_eq!("Foo", "FoO".capitalized(Default::default()));
		assert_eq!("Foo", "foo".capitalized(Default::default()));
		assert_eq!("Foo", "foO".capitalized(Default::default()));
	}

	#[test]
	fn capitalized_allocation() {
		assert_borrowed!("Foo".capitalized(Default::default()));
		assert_borrowed!("Foo-foo-æßð".capitalized(Default::default()));

		assert_owned!("fOOOOO".capitalized(Default::default()));
		assert_owned!("REEEeE".capitalized(Default::default()));
	}

	#[test]
	fn camel() {
		assert_eq!("FooBar", "FooBar".camel(Default::default(), Camel::Upper, Default::default()));
		assert_eq!("Foo", "Foo".camel(Default::default(), Camel::Upper, Default::default()));
		assert_eq!("FooBar", "Foo-bar".camel(Default::default(), Camel::Upper, Default::default()));
		assert_eq!("FooBar", "foo_bar".camel(Default::default(), Camel::Upper, Default::default()));

		assert_eq!("fooBar", "foo-Bar".camel(Default::default(), Camel::Lower, Default::default()));
		assert_eq!("fooBarBaz", "foo_bar-baz".camel(Default::default(), Camel::Lower, Default::default()));
		assert_eq!("fooBar", "fooBar".camel(Default::default(), Camel::Lower, Default::default()));
		assert_eq!("foo", "foo".camel(Default::default(), Camel::Lower, Default::default()));
	}

	#[test]
	fn camel_allocation() {
		assert_borrowed!("FooBar".camel(Default::default(), Camel::Upper, Default::default()));
		assert_borrowed!("fooBar".camel(Default::default(), Camel::Lower, Default::default()));
	}

	#[test]
	fn separated() {
		assert_eq!("foo_bar", "foo_bar".separated(Separator('_'), Default::default()));
		assert_eq!("foo-bar-baz", "foo_bar_baz".separated(Separator('-'), Default::default()));
	}

	#[test]
	fn separated_allocation() {
		assert_borrowed!("foo_bar".separated(Separator('_'), Default::default()));
		assert_borrowed!("foo-bar".separated(Separator('-'), Default::default()));
		assert_borrowed!("foo@bar".separated(Separator('@'), Default::default()));

		assert_owned!("foo_bar".separated(Separator('@'), Default::default()));
		assert_owned!("foo-bar".separated(Separator('_'), Default::default()));
		assert_owned!("foo@bar".separated(Separator('-'), Default::default()));
	}

	#[test]
	fn header() {
		assert_eq!("Foo", "Foo".header(Default::default()));
		assert_eq!("Foo-Bar", "Foo-Bar".header(Default::default()));
		assert_eq!("Foo-Bar-Baz", "Foo-Bar-Baz".header(Default::default()));
		assert_eq!("MIME-Type", "MIME-Type".header(Default::default()));
		assert_eq!("Foo", "foo".header(Default::default()));
		assert_eq!("Foo-Bar", "foo-bar".header(Default::default()));
		assert_eq!("Foo-Bar-Baz", "foo-bar-baz".header(Default::default()));
	}

	#[test]
	fn header_allocation() {
		assert_borrowed!("Foo".header(Default::default()));
		assert_borrowed!("Foo-Bar".header(Default::default()));
		assert_borrowed!("Foo-Bar-Baz".header(Default::default()));
		assert_borrowed!("MIME-Type".header(Default::default()));

		assert_owned!("foo".header(Default::default()));
		assert_owned!("foo-bar".header(Default::default()));
		assert_owned!("foo-bar-baz".header(Default::default()));
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
