use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, TryFromBigIntError};
use num_traits::{Signed, Zero};

use crate::{
	lexical,
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

impl Add for NonNegativeInteger {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl Sub for NonNegativeInteger {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Mul for NonNegativeInteger {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl Div for NonNegativeInteger {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self(self.0 / rhs.0)
	}
}

macro_rules! impl_arithmetic {
	{
		$( $ty:ty ),*
	} => {
		$(
			impl Add<$ty> for NonNegativeInteger {
				type Output = Self;

				fn add(self, rhs: $ty) -> Self::Output {
					let n: BigInt = self.0 + rhs;
					assert!(!n.is_negative());
					Self(n)
				}
			}

			impl Sub<$ty> for NonNegativeInteger {
				type Output = Self;

				fn sub(self, rhs: $ty) -> Self::Output {
					let n: BigInt = self.0 - rhs;
					assert!(!n.is_negative());
					Self(n)
				}
			}

			impl Mul<$ty> for NonNegativeInteger {
				type Output = Self;

				fn mul(self, rhs: $ty) -> Self::Output {
					let n: BigInt = self.0 * rhs;
					assert!(!n.is_negative());
					Self(n)
				}
			}

			impl Div<$ty> for NonNegativeInteger {
				type Output = Self;

				fn div(self, rhs: $ty) -> Self::Output {
					let n: BigInt = self.0 / rhs;
					assert!(!n.is_negative());
					Self(n)
				}
			}
		)*
	};
}

impl_arithmetic!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

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

impl XsdDatatype for PositiveInteger {
	fn type_(&self) -> Datatype {
		NonNegativeIntegerDatatype::PositiveInteger.into()
	}
}
