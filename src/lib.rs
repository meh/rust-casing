#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use std::borrow::{ToOwned, Cow};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Camel {
	Upper,
	Lower,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Locale {
	None,
}

impl Default for Locale {
	fn default() -> Self {
		Locale::None
	}
}

pub trait Casing: ToOwned {
	fn upper(&self, Locale) -> Cow<Self>;

	fn lower(&self, Locale) -> Cow<Self>;

	fn capitalized(&self, Locale) -> Cow<Self>;

	fn camel(&self, mode: Camel, Locale) -> Cow<Self>;

	fn snake(&self, Locale) -> Cow<Self>;

	fn dashed(&self, Locale) -> Cow<Self>;

	fn header(&self, Locale) -> Cow<Self>;
}

mod unicode;
mod ascii;
