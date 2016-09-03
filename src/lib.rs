#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use std::borrow::{ToOwned, Cow};
use std::ops::Deref;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Camel {
	Upper,
	Lower,
}

static DEFAULT_SEPARATOR_CHAR_MULTI: &'static [char] = &['-', '_'];
static DEFAULT_SEPARATOR_U8_MULTI:   &'static [u8] = &[b'-', b'_'];

/// Wrapper for a string separator.
#[derive(Eq, PartialEq, Copy, Debug, Clone)]
pub struct Separator<T: Copy + Eq>(pub T);

impl Default for Separator<char> {
	fn default() -> Self {
		Separator('_')
	}
}

impl Default for Separator<u8> {
	fn default() -> Self {
		Separator(b'_')
	}
}

impl<'a> Default for Separator<&'a [char]> {
	fn default() -> Self {
		Separator(DEFAULT_SEPARATOR_CHAR_MULTI)
	}
}

impl<'a> Default for Separator<&'a [u8]> {
	fn default() -> Self {
		Separator(DEFAULT_SEPARATOR_U8_MULTI)
	}
}

impl<T: Copy + Eq> Deref for Separator<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/// A specific locale.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Locale {
	None,
}

impl Default for Locale {
	fn default() -> Self {
		Locale::None
	}
}

/// Trait with casing extensions.
pub trait Casing: ToOwned {
	type Character: Copy + Eq;

	/// Turns `Self` to upper case avoiding allocations if nothing would change.
	fn upper(&self, Locale) -> Cow<Self>;

	/// Turns `Self` to lower case avoiding allocations if nothing would change.
	fn lower(&self, Locale) -> Cow<Self>;

	/// Turns `Self` to its capitalized version, turning the first character to
	/// upper case and the rest to lower case.
	fn capitalized(&self, Locale) -> Cow<Self>;

	/// Turns `Self` to camel case using the passed `Separator` to know which
	/// symbols should mark a new word.
	fn camel(&self, separators: Separator<&[Self::Character]>, mode: Camel, Locale) -> Cow<Self>;

	/// Turns `Self` to a case separated by the given separator, using
	/// `Separator('_')` would turn to snake case, using `Separator('-')` would
	/// turn to dashed case.
	fn separated(&self, separator: Separator<Self::Character>, Locale) -> Cow<Self>;

	/// Turns `Self` to header case, where each word is separated by `'-'` and
	/// starts with an upper case character. Upper case characters after the
	/// first are not lower cased.
	fn header(&self, Locale) -> Cow<Self>;
}

mod unicode;
mod ascii;
