use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	impl_integer_arithmetic, lexical,
	value::decimal::{U16_MAX, U32_MAX, U64_MAX, U8_MAX},
	Datatype, Integer, NonNegativeIntegerDatatype, UnsignedIntDatatype, UnsignedLongDatatype,
	UnsignedShortDatatype, XsdDatatype,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NonNegativeInteger(BigInt);

impl NonNegativeInteger {
	/// Create a new non negative integer from a `BigInt`.
	///
	/// # Safety
	///
	/// The input number must be non negative.
	pub unsafe fn new_unchecked(n: BigInt) -> Self {
		Self(n)
	}

	#[inline(always)]
	pub fn into_big_int(self) -> BigInt {
		self.0
	}

	#[inline(always)]
	pub fn zero() -> Self {
		Self(BigInt::zero())
	}

	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	pub fn non_negative_integer_type(&self) -> Option<NonNegativeIntegerDatatype> {
		if self.0 > BigInt::zero() {
			if self.0 <= *U8_MAX {
				Some(UnsignedShortDatatype::UnsignedByte.into())
			} else if self.0 <= *U16_MAX {
				Some(UnsignedIntDatatype::UnsignedShort(None).into())
			} else if self.0 <= *U32_MAX {
				Some(UnsignedLongDatatype::UnsignedInt(None).into())
			} else if self.0 <= *U64_MAX {
				Some(NonNegativeIntegerDatatype::UnsignedLong(None))
			} else {
				Some(NonNegativeIntegerDatatype::PositiveInteger)
			}
		} else {
			Some(UnsignedShortDatatype::UnsignedByte.into())
		}
	}

	/// Returns a lexical representation of this non negative integer.
	#[inline(always)]
	pub fn lexical_representation(&self) -> lexical::NonNegativeIntegerBuf {
		unsafe {
			// This is safe because the `Display::fmt` method matches the
			// XSD lexical representation.
			lexical::NonNegativeIntegerBuf::new_unchecked(format!("{}", self))
		}
	}
}

impl XsdDatatype for NonNegativeInteger {
	fn type_(&self) -> Datatype {
		self.non_negative_integer_type().into()
	}
}

impl From<NonNegativeInteger> for BigInt {
	fn from(value: NonNegativeInteger) -> Self {
		value.0
	}
}

impl<'a> From<&'a lexical::NonNegativeInteger> for NonNegativeInteger {
	#[inline(always)]
	fn from(value: &'a lexical::NonNegativeInteger) -> Self {
		Self(value.as_str().parse().unwrap())
	}
}

impl From<lexical::NonNegativeIntegerBuf> for NonNegativeInteger {
	#[inline(always)]
	fn from(value: lexical::NonNegativeIntegerBuf) -> Self {
		value.as_non_negative_integer().into()
	}
}

impl FromStr for NonNegativeInteger {
	type Err = lexical::InvalidNonNegativeInteger;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::NonNegativeInteger::new(s)?;
		Ok(l.into())
	}
}

impl fmt::Display for NonNegativeInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl AsRef<BigInt> for NonNegativeInteger {
	#[inline(always)]
	fn as_ref(&self) -> &BigInt {
		&self.0
	}
}

impl Borrow<BigInt> for NonNegativeInteger {
	#[inline(always)]
	fn borrow(&self) -> &BigInt {
		&self.0
	}
}

#[derive(Debug, thiserror::Error)]
#[error("integer {0} is negative")]
pub struct IntegerIsNegative(Integer);

impl TryFrom<Integer> for NonNegativeInteger {
	type Error = IntegerIsNegative;

	fn try_from(value: Integer) -> Result<Self, Self::Error> {
		if value.is_negative() {
			Err(IntegerIsNegative(value))
		} else {
			Ok(Self(value.into()))
		}
	}
}

macro_rules! from {
	{ $( $ty:ty ),* } => {
		$(
			impl From<$ty> for NonNegativeInteger {
				fn from(value: $ty) -> Self {
					Self(value.into())
				}
			}
		)*
	};
}

from!(u8, u16, u32, u64, usize);

#[derive(Debug, thiserror::Error)]
#[error("integer out of supported bounds: {0}")]
pub struct NonNegativeIntegerOutOfTargetBounds(pub NonNegativeInteger);

macro_rules! try_into {
	{ $( $ty:ty ),* } => {
		$(
			impl TryFrom<NonNegativeInteger> for $ty {
				type Error = NonNegativeIntegerOutOfTargetBounds;

				fn try_from(value: NonNegativeInteger) -> Result<Self, Self::Error> {
					value.0.try_into().map_err(|e: TryFromBigIntError<BigInt>| NonNegativeIntegerOutOfTargetBounds(NonNegativeInteger(e.into_original())))
				}
			}
		)*
	};
}

try_into!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl_integer_arithmetic!(
	for NonNegativeInteger where r ( !r.is_negative() ) {
		Integer [.0],
		NonNegativeInteger [.0],
		PositiveInteger [.0],
		super::NonPositiveInteger [.into_big_int()],
		super::NegativeInteger [.into_big_int()],
		i8,
		i16,
		i32,
		i64,
		isize,
		u8,
		u16,
		u32,
		u64,
		usize
	}
);

pub type UnsignedLong = u64;

pub trait XsdUnsignedLong {
	fn unsigned_long_type(&self) -> Option<UnsignedLongDatatype>;
}

impl XsdUnsignedLong for UnsignedLong {
	fn unsigned_long_type(&self) -> Option<UnsignedLongDatatype> {
		if *self <= u8::MAX as u64 {
			Some(UnsignedShortDatatype::UnsignedByte.into())
		} else if *self <= u16::MAX as u64 {
			Some(UnsignedIntDatatype::UnsignedShort(None).into())
		} else if *self <= u32::MAX as u64 {
			Some(UnsignedLongDatatype::UnsignedInt(None))
		} else {
			None
		}
	}
}

impl XsdDatatype for UnsignedLong {
	fn type_(&self) -> Datatype {
		self.unsigned_long_type().into()
	}
}

pub type UnsignedInt = u32;

pub trait XsdUnsignedInt {
	fn unsigned_int_type(&self) -> Option<UnsignedIntDatatype>;
}

impl XsdUnsignedInt for UnsignedInt {
	fn unsigned_int_type(&self) -> Option<UnsignedIntDatatype> {
		if *self <= u8::MAX as u32 {
			Some(UnsignedShortDatatype::UnsignedByte.into())
		} else if *self <= u16::MAX as u32 {
			Some(UnsignedIntDatatype::UnsignedShort(None))
		} else {
			None
		}
	}
}

impl XsdDatatype for UnsignedInt {
	fn type_(&self) -> Datatype {
		self.unsigned_int_type().into()
	}
}

pub type UnsignedShort = u16;

pub trait XsdUnsignedShort {
	fn unsigned_short_type(&self) -> Option<UnsignedShortDatatype>;
}

impl XsdUnsignedShort for UnsignedShort {
	fn unsigned_short_type(&self) -> Option<UnsignedShortDatatype> {
		if *self <= u8::MAX as u16 {
			Some(UnsignedShortDatatype::UnsignedByte)
		} else {
			None
		}
	}
}

impl XsdDatatype for UnsignedShort {
	fn type_(&self) -> Datatype {
		self.unsigned_short_type().into()
	}
}

pub type UnsignedByte = u8;

impl XsdDatatype for UnsignedByte {
	fn type_(&self) -> Datatype {
		UnsignedShortDatatype::UnsignedByte.into()
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PositiveInteger(BigInt);

impl PositiveInteger {
	/// Creates a new positive integer from the given `BigInt`.
	/// 
	/// # Safety
	/// 
	/// The input value *must* but a positive integer.
	pub unsafe fn new_unchecked(n: BigInt) -> Self {
		Self(n)
	}

	pub fn into_big_int(self) -> BigInt {
		self.0
	}

	pub fn is_one(&self) -> bool {
		matches!(u8::try_from(&self.0), Ok(1))
	}
}

impl XsdDatatype for PositiveInteger {
	fn type_(&self) -> Datatype {
		NonNegativeIntegerDatatype::PositiveInteger.into()
	}
}

impl fmt::Display for PositiveInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl_integer_arithmetic!(
	for PositiveInteger where r ( r.is_positive() ) {
		Integer [.0],
		NonNegativeInteger [.0],
		PositiveInteger [.0],
		super::NonPositiveInteger [.into_big_int()],
		super::NegativeInteger [.into_big_int()],
		i8,
		i16,
		i32,
		i64,
		isize,
		u8,
		u16,
		u32,
		u64,
		usize
	}
);
