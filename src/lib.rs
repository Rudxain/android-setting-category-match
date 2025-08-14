#![no_std]
use core::hint::assert_unchecked;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AndroidSettingCategory {
	System = 0,
	Secure = 1,
	Global = 2,
}

impl AndroidSettingCategory {
	#[must_use]
	pub const fn new(s: &str) -> Option<Self> {
		match s.as_bytes() {
			b"system" => Some(Self::System),
			b"secure" => Some(Self::Secure),
			b"global" => Some(Self::Global),
			_ => None,
		}
	}

	/// # Safety
	/// Assumes the slice is one of these ASCII strings:
	/// - `"system"`
	/// - `"secure"`
	/// - `"global"`
	///
	/// Anything else is immediate UB
	#[expect(unsafe_op_in_unsafe_fn)]
	#[must_use]
	pub const unsafe fn new_unchecked(s: &[u8]) -> Self {
		// allow the optimizer to do its job
		Self::new(str::from_utf8_unchecked(s)).unwrap_unchecked();
		// guarantee that bounds-check is eliminated
		assert_unchecked(s.len() == 6);

		// avoid comparing all bytes
		let i = s[3] & 0b11;

		// give extra info to optimizer
		assert_unchecked(i < 3);

		core::mem::transmute(i)
	}
}
