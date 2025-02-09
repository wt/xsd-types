//! This crate aims at providing safe representations
//! of [XSD built-in data types](https://www.w3.org/TR/xmlschema-2/#built-in-datatypes).
//! For now, only numeric types are implemented.
use iref::Iri;
use static_iref::iri;

pub mod lexical;
pub mod value;

use lexical::{Lexical, LexicalFormOf};
pub use value::*;

/// XSD primitive datatype.
pub enum PrimitiveDatatype {
	String,
	Boolean,
	Decimal,
	Float,
	Double,
	Duration,
	DateTime,
	Time,
	Date,
	GYearMonth,
	GYear,
	GMonthDay,
	GDay,
	GMonth,
	HexBinary,
	Base64Binary,
	AnyUri,
	QName,
	Notation,
}

macro_rules! impl_from {
	{
		$ty:ty {
			$($input:ident : $from_ty:ty => Self::$variant:ident($output:expr)),*
		}
	} => {
		$(
			impl From<$from_ty> for $ty {
				fn from($input: $from_ty) -> Self {
					Self::$variant(Some($output))
				}
			}

			impl From<Option<$from_ty>> for $ty {
				fn from(input_opt: Option<$from_ty>) -> Self {
					Self::$variant(input_opt.map(|$input| $output))
				}
			}
		)*
	};
}

pub const XSD_DURATION: &Iri = iri!("http://www.w3.org/2001/XMLSchema#duration");
pub const XSD_DATE_TIME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#dateTime");
pub const XSD_TIME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#time");
pub const XSD_DATE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#date");
pub const XSD_G_YEAR_MONTH: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gYearMonth");
pub const XSD_G_YEAR: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gYear");
pub const XSD_G_MONTH_DAY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gMonthDay");
pub const XSD_G_DAY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gDay");
pub const XSD_G_MONTH: &Iri = iri!("http://www.w3.org/2001/XMLSchema#gMonth");
pub const XSD_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#string");
pub const XSD_BOOLEAN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#boolean");
pub const XSD_BASE64_BINARY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#base64Binary");
pub const XSD_HEX_BINARY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#hexBinary");
pub const XSD_FLOAT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#float");
pub const XSD_DECIMAL: &Iri = iri!("http://www.w3.org/2001/XMLSchema#decimal");
pub const XSD_DOUBLE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#double");
pub const XSD_ANY_URI: &Iri = iri!("http://www.w3.org/2001/XMLSchema#anyURI");
pub const XSD_Q_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#QName");
pub const XSD_NOTATION: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NOTATION");
pub const XSD_NORMALIZED_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#normalizedString");
pub const XSD_TOKEN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#token");
pub const XSD_LANGUAGE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#language");
pub const XSD_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#Name");
pub const XSD_NMTOKEN: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NMTOKEN");
pub const XSD_NC_NAME: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NCName");
pub const XSD_NMTOKENS: &Iri = iri!("http://www.w3.org/2001/XMLSchema#NMTOKENS");
pub const XSD_ID: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ID");
pub const XSD_IDREF: &Iri = iri!("http://www.w3.org/2001/XMLSchema#IDREF");
pub const XSD_ENTITY: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ENTITY");
pub const XSD_IDREFS: &Iri = iri!("http://www.w3.org/2001/XMLSchema#IDREFS");
pub const XSD_ENTITIES: &Iri = iri!("http://www.w3.org/2001/XMLSchema#ENTITIES");
pub const XSD_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#integer");
pub const XSD_NON_POSITIVE_INTEGER: &Iri =
	iri!("http://www.w3.org/2001/XMLSchema#nonPositiveInteger");
pub const XSD_NEGATIVE_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#negativeInteger");
pub const XSD_LONG: &Iri = iri!("http://www.w3.org/2001/XMLSchema#long");
pub const XSD_INT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#int");
pub const XSD_SHORT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#short");
pub const XSD_BYTE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#byte");
pub const XSD_NON_NEGATIVE_INTEGER: &Iri =
	iri!("http://www.w3.org/2001/XMLSchema#nonNegativeInteger");
pub const XSD_UNSIGNED_LONG: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedLong");
pub const XSD_UNSIGNED_INT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedInt");
pub const XSD_UNSIGNED_SHORT: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedShort");
pub const XSD_UNSIGNED_BYTE: &Iri = iri!("http://www.w3.org/2001/XMLSchema#unsignedByte");
pub const XSD_POSITIVE_INTEGER: &Iri = iri!("http://www.w3.org/2001/XMLSchema#positiveInteger");

/// XSD datatype.
pub enum Datatype {
	String(Option<StringDatatype>),
	Boolean,
	Decimal(Option<DecimalDatatype>),
	Float,
	Double,
	Duration,
	DateTime,
	Time,
	Date,
	GYearMonth,
	GYear,
	GMonthDay,
	GDay,
	GMonth,
	HexBinary,
	Base64Binary,
	AnyUri,
	QName,
	Notation,
}

impl Datatype {
	#[allow(clippy::if_same_then_else)] // until TODOs are resolved.
	pub fn from_iri(iri: &Iri) -> Option<Self> {
		// TODO built-in types derived by list (NMTOKENS, IDREFS, ENTITIES).
		if iri == XSD_DURATION {
			Some(Self::Duration)
		} else if iri == XSD_DATE_TIME {
			Some(Self::DateTime)
		} else if iri == XSD_TIME {
			Some(Self::Time)
		} else if iri == XSD_DATE {
			Some(Self::Date)
		} else if iri == XSD_G_YEAR_MONTH {
			Some(Self::GYearMonth)
		} else if iri == XSD_G_YEAR {
			Some(Self::GYear)
		} else if iri == XSD_G_MONTH_DAY {
			Some(Self::GMonthDay)
		} else if iri == XSD_G_DAY {
			Some(Self::GDay)
		} else if iri == XSD_G_MONTH {
			Some(Self::GMonth)
		} else if iri == XSD_STRING {
			Some(Self::String(None))
		} else if iri == XSD_BOOLEAN {
			Some(Self::Boolean)
		} else if iri == XSD_BASE64_BINARY {
			Some(Self::Base64Binary)
		} else if iri == XSD_HEX_BINARY {
			Some(Self::HexBinary)
		} else if iri == XSD_FLOAT {
			Some(Self::Float)
		} else if iri == XSD_DECIMAL {
			Some(Self::Decimal(None))
		} else if iri == XSD_DOUBLE {
			Some(Self::Double)
		} else if iri == XSD_ANY_URI {
			Some(Self::AnyUri)
		} else if iri == XSD_Q_NAME {
			Some(Self::QName)
		} else if iri == XSD_NOTATION {
			Some(Self::Notation)
		} else if iri == XSD_NORMALIZED_STRING {
			Some(Self::String(Some(StringDatatype::NormalizedString(None))))
		} else if iri == XSD_TOKEN {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(None),
			)))))
		} else if iri == XSD_LANGUAGE {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(Some(TokenDatatype::Language)),
			)))))
		} else if iri == XSD_NAME {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(Some(TokenDatatype::Name(None))),
			)))))
		} else if iri == XSD_NMTOKEN {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(Some(TokenDatatype::NMToken)),
			)))))
		} else if iri == XSD_NMTOKENS {
			None // TODO
		} else if iri == XSD_NC_NAME {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(Some(TokenDatatype::Name(Some(
					NameDatatype::NCName(None),
				)))),
			)))))
		} else if iri == XSD_ID {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(Some(TokenDatatype::Name(Some(
					NameDatatype::NCName(Some(NCNameDatatype::Id)),
				)))),
			)))))
		} else if iri == XSD_IDREF {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(Some(TokenDatatype::Name(Some(
					NameDatatype::NCName(Some(NCNameDatatype::IdRef)),
				)))),
			)))))
		} else if iri == XSD_ENTITY {
			Some(Self::String(Some(StringDatatype::NormalizedString(Some(
				NormalizedStringDatatype::Token(Some(TokenDatatype::Name(Some(
					NameDatatype::NCName(Some(NCNameDatatype::Entity)),
				)))),
			)))))
		} else if iri == XSD_IDREFS {
			None // TODO
		} else if iri == XSD_ENTITIES {
			None // TODO
		} else if iri == XSD_INTEGER {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(None))))
		} else if iri == XSD_NON_POSITIVE_INTEGER {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonPositiveInteger(None),
			)))))
		} else if iri == XSD_NEGATIVE_INTEGER {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonPositiveInteger(Some(
					NonPositiveIntegerDatatype::NegativeInteger,
				)),
			)))))
		} else if iri == XSD_LONG {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(None),
			)))))
		} else if iri == XSD_INT {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(None))),
			)))))
		} else if iri == XSD_SHORT {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(Some(IntDatatype::Short(None))))),
			)))))
		} else if iri == XSD_BYTE {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::Long(Some(LongDatatype::Int(Some(IntDatatype::Short(Some(
					ShortDatatype::Byte,
				)))))),
			)))))
		} else if iri == XSD_NON_NEGATIVE_INTEGER {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(None),
			)))))
		} else if iri == XSD_UNSIGNED_LONG {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(None),
				)),
			)))))
		} else if iri == XSD_UNSIGNED_INT {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(None),
					)),
				)),
			)))))
		} else if iri == XSD_UNSIGNED_SHORT {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(Some(
							UnsignedIntDatatype::UnsignedShort(None),
						)),
					)),
				)),
			)))))
		} else if iri == XSD_UNSIGNED_BYTE {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::UnsignedLong(Some(
						UnsignedLongDatatype::UnsignedInt(Some(
							UnsignedIntDatatype::UnsignedShort(Some(
								UnsignedShortDatatype::UnsignedByte,
							)),
						)),
					)),
				)),
			)))))
		} else if iri == XSD_POSITIVE_INTEGER {
			Some(Self::Decimal(Some(DecimalDatatype::Integer(Some(
				IntegerDatatype::NonNegativeInteger(Some(
					NonNegativeIntegerDatatype::PositiveInteger,
				)),
			)))))
		} else {
			None
		}
	}

	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::String(None) => XSD_STRING,
			Self::String(Some(t)) => t.iri(),
			Self::Boolean => XSD_BOOLEAN,
			Self::Decimal(None) => XSD_DECIMAL,
			Self::Decimal(Some(t)) => t.iri(),
			Self::Float => XSD_FLOAT,
			Self::Double => XSD_DOUBLE,
			Self::Duration => XSD_DURATION,
			Self::DateTime => XSD_DATE_TIME,
			Self::Time => XSD_TIME,
			Self::Date => XSD_DATE,
			Self::GYearMonth => XSD_G_YEAR_MONTH,
			Self::GYear => XSD_G_YEAR,
			Self::GMonthDay => XSD_G_MONTH_DAY,
			Self::GDay => XSD_G_DAY,
			Self::GMonth => XSD_G_MONTH,
			Self::HexBinary => XSD_HEX_BINARY,
			Self::Base64Binary => XSD_BASE64_BINARY,
			Self::AnyUri => XSD_ANY_URI,
			Self::QName => XSD_Q_NAME,
			Self::Notation => XSD_NOTATION,
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::String(None) => Ok(Value::String(value.to_owned())),
			Self::String(Some(_t)) => todo!(),
			Self::Boolean => ParseRdf::parse_rdf(value)
				.map(Value::Boolean)
				.map_err(|_| ParseError),
			Self::Decimal(None) => ParseRdf::parse_rdf(value)
				.map(Value::Decimal)
				.map_err(|_| ParseError),
			Self::Decimal(Some(t)) => t.parse(value),
			Self::Float => ParseRdf::parse_rdf(value)
				.map(Value::Float)
				.map_err(|_| ParseError),
			Self::Double => ParseRdf::parse_rdf(value)
				.map(Value::Double)
				.map_err(|_| ParseError),
			Self::Duration => todo!(),
			Self::DateTime => ParseRdf::parse_rdf(value)
				.map(Value::DateTime)
				.map_err(|_| ParseError),
			Self::Time => todo!(),
			Self::Date => todo!(),
			Self::GYearMonth => todo!(),
			Self::GYear => todo!(),
			Self::GMonthDay => todo!(),
			Self::GDay => todo!(),
			Self::GMonth => todo!(),
			Self::HexBinary => ParseRdf::parse_rdf(value)
				.map(Value::HexBinary)
				.map_err(|_| ParseError),
			Self::Base64Binary => ParseRdf::parse_rdf(value)
				.map(Value::Base64Binary)
				.map_err(|_| ParseError),
			Self::AnyUri => ParseRdf::parse_rdf(value)
				.map(Value::AnyUri)
				.map_err(|_| ParseError),
			Self::QName => todo!(),
			Self::Notation => todo!(),
		}
	}
}

pub struct ParseError;

impl AsRef<Iri> for Datatype {
	fn as_ref(&self) -> &Iri {
		self.iri()
	}
}

impl_from!(Datatype {
	ty: StringDatatype => Self::String(ty),
	ty: DecimalDatatype => Self::Decimal(ty),
	ty: IntegerDatatype => Self::Decimal(ty.into()),
	ty: NonPositiveIntegerDatatype => Self::Decimal(ty.into()),
	ty: LongDatatype => Self::Decimal(ty.into()),
	ty: IntDatatype => Self::Decimal(ty.into()),
	ty: ShortDatatype => Self::Decimal(ty.into()),
	ty: NonNegativeIntegerDatatype => Self::Decimal(ty.into()),
	ty: UnsignedLongDatatype => Self::Decimal(ty.into()),
	ty: UnsignedIntDatatype => Self::Decimal(ty.into()),
	ty: UnsignedShortDatatype => Self::Decimal(ty.into())
});

/// Parse a value directly from its RDF lexical form.
pub trait ParseRdf: Sized {
	type LexicalForm: LexicalFormOf<Self> + ?Sized;

	fn parse_rdf(lexical_value: &str) -> ParseRdfResult<Self, Self::LexicalForm> {
		Self::LexicalForm::parse(lexical_value)
			.map_err(ParseRdfError::InvalidLexicalForm)?
			.try_as_value()
			.map_err(ParseRdfError::InvalidValue)
	}
}

pub type ParseRdfResult<T, L> =
	Result<T, ParseRdfError<<L as Lexical>::Error, <L as LexicalFormOf<T>>::ValueError>>;

pub enum ParseRdfError<L, V> {
	InvalidLexicalForm(L),
	InvalidValue(V),
}

/// Datatype derived from `xsd:string`.
pub enum StringDatatype {
	NormalizedString(Option<NormalizedStringDatatype>),
}

impl StringDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NormalizedString(None) => XSD_NORMALIZED_STRING,
			Self::NormalizedString(Some(t)) => t.iri(),
		}
	}
}

pub enum NormalizedStringDatatype {
	Token(Option<TokenDatatype>),
}

impl NormalizedStringDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Token(None) => XSD_TOKEN,
			Self::Token(Some(t)) => t.iri(),
		}
	}
}

pub enum TokenDatatype {
	Language,
	NMToken,
	Name(Option<NameDatatype>),
}

impl TokenDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Language => XSD_LANGUAGE,
			Self::NMToken => XSD_NMTOKEN,
			Self::Name(None) => XSD_NAME,
			Self::Name(Some(t)) => t.iri(),
		}
	}
}

pub enum NameDatatype {
	NCName(Option<NCNameDatatype>),
}

impl NameDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NCName(None) => XSD_NC_NAME,
			Self::NCName(Some(t)) => t.iri(),
		}
	}
}

pub enum NCNameDatatype {
	Id,
	IdRef,
	Entity,
}

impl NCNameDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Id => XSD_ID,
			Self::IdRef => XSD_IDREF,
			Self::Entity => XSD_ENTITY,
		}
	}
}

/// Datatype derived from `xsd:decimal`.
pub enum DecimalDatatype {
	Integer(Option<IntegerDatatype>),
}

impl DecimalDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Integer(None) => XSD_INTEGER,
			Self::Integer(Some(t)) => t.iri(),
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::Integer(None) => ParseRdf::parse_rdf(value)
				.map(Value::Integer)
				.map_err(|_| ParseError),
			Self::Integer(Some(t)) => t.parse(value),
		}
	}
}

impl_from!(DecimalDatatype {
	ty: IntegerDatatype => Self::Integer(ty),
	ty: NonPositiveIntegerDatatype => Self::Integer(ty.into()),
	ty: LongDatatype => Self::Integer(ty.into()),
	ty: IntDatatype => Self::Integer(ty.into()),
	ty: ShortDatatype => Self::Integer(ty.into()),
	ty: NonNegativeIntegerDatatype => Self::Integer(ty.into()),
	ty: UnsignedLongDatatype => Self::Integer(ty.into()),
	ty: UnsignedIntDatatype => Self::Integer(ty.into()),
	ty: UnsignedShortDatatype => Self::Integer(ty.into())
});

pub enum IntegerDatatype {
	NonPositiveInteger(Option<NonPositiveIntegerDatatype>),
	Long(Option<LongDatatype>),
	NonNegativeInteger(Option<NonNegativeIntegerDatatype>),
}

impl IntegerDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NonPositiveInteger(None) => XSD_NON_POSITIVE_INTEGER,
			Self::NonPositiveInteger(Some(t)) => t.iri(),
			Self::Long(None) => XSD_LONG,
			Self::Long(Some(t)) => t.iri(),
			Self::NonNegativeInteger(None) => XSD_NON_NEGATIVE_INTEGER,
			Self::NonNegativeInteger(Some(t)) => t.iri(),
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::NonPositiveInteger(None) => ParseRdf::parse_rdf(value)
				.map(Value::NonPositiveInteger)
				.map_err(|_| ParseError),
			Self::NonPositiveInteger(Some(t)) => t.parse(value),
			Self::Long(None) => ParseRdf::parse_rdf(value)
				.map(Value::Long)
				.map_err(|_| ParseError),
			Self::Long(Some(t)) => t.parse(value),
			Self::NonNegativeInteger(None) => ParseRdf::parse_rdf(value)
				.map(Value::NonNegativeInteger)
				.map_err(|_| ParseError),
			Self::NonNegativeInteger(Some(t)) => t.parse(value),
		}
	}
}

impl_from!(IntegerDatatype {
	ty: NonPositiveIntegerDatatype => Self::NonPositiveInteger(ty),
	ty: LongDatatype => Self::Long(ty),
	ty: IntDatatype => Self::Long(ty.into()),
	ty: ShortDatatype => Self::Long(ty.into()),
	ty: NonNegativeIntegerDatatype => Self::NonNegativeInteger(ty),
	ty: UnsignedLongDatatype => Self::NonNegativeInteger(ty.into()),
	ty: UnsignedIntDatatype => Self::NonNegativeInteger(ty.into()),
	ty: UnsignedShortDatatype => Self::NonNegativeInteger(ty.into())
});

pub enum NonPositiveIntegerDatatype {
	NegativeInteger,
}

impl NonPositiveIntegerDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::NegativeInteger => XSD_NEGATIVE_INTEGER,
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::NegativeInteger => ParseRdf::parse_rdf(value)
				.map(Value::NegativeInteger)
				.map_err(|_| ParseError),
		}
	}
}

pub enum LongDatatype {
	Int(Option<IntDatatype>),
}

impl LongDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Int(None) => XSD_INT,
			Self::Int(Some(t)) => t.iri(),
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::Int(None) => ParseRdf::parse_rdf(value)
				.map(Value::Int)
				.map_err(|_| ParseError),
			Self::Int(Some(t)) => t.parse(value),
		}
	}
}

impl_from!(LongDatatype {
	ty: IntDatatype => Self::Int(ty),
	ty: ShortDatatype => Self::Int(ty.into())
});

pub enum IntDatatype {
	Short(Option<ShortDatatype>),
}

impl IntDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Short(None) => XSD_SHORT,
			Self::Short(Some(t)) => t.iri(),
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::Short(None) => ParseRdf::parse_rdf(value)
				.map(Value::Short)
				.map_err(|_| ParseError),
			Self::Short(Some(t)) => t.parse(value),
		}
	}
}

impl_from!(IntDatatype {
	ty: ShortDatatype => Self::Short(ty)
});

pub enum ShortDatatype {
	Byte,
}

impl ShortDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::Byte => XSD_BYTE,
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::Byte => ParseRdf::parse_rdf(value)
				.map(Value::Byte)
				.map_err(|_| ParseError),
		}
	}
}

pub enum NonNegativeIntegerDatatype {
	UnsignedLong(Option<UnsignedLongDatatype>),
	PositiveInteger,
}

impl NonNegativeIntegerDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedLong(None) => XSD_UNSIGNED_LONG,
			Self::UnsignedLong(Some(t)) => t.iri(),
			Self::PositiveInteger => XSD_POSITIVE_INTEGER,
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::UnsignedLong(None) => ParseRdf::parse_rdf(value)
				.map(Value::UnsignedLong)
				.map_err(|_| ParseError),
			Self::UnsignedLong(Some(t)) => t.parse(value),
			Self::PositiveInteger => ParseRdf::parse_rdf(value)
				.map(Value::PositiveInteger)
				.map_err(|_| ParseError),
		}
	}
}

impl_from!(NonNegativeIntegerDatatype {
	ty: UnsignedLongDatatype => Self::UnsignedLong(ty),
	ty: UnsignedIntDatatype => Self::UnsignedLong(ty.into()),
	ty: UnsignedShortDatatype => Self::UnsignedLong(ty.into())
});

pub enum UnsignedLongDatatype {
	UnsignedInt(Option<UnsignedIntDatatype>),
}

impl UnsignedLongDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedInt(None) => XSD_UNSIGNED_INT,
			Self::UnsignedInt(Some(t)) => t.iri(),
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::UnsignedInt(None) => ParseRdf::parse_rdf(value)
				.map(Value::UnsignedInt)
				.map_err(|_| ParseError),
			Self::UnsignedInt(Some(t)) => t.parse(value),
		}
	}
}

impl_from!(UnsignedLongDatatype {
	ty: UnsignedIntDatatype => Self::UnsignedInt(ty),
	ty: UnsignedShortDatatype => Self::UnsignedInt(ty.into())
});

pub enum UnsignedIntDatatype {
	UnsignedShort(Option<UnsignedShortDatatype>),
}

impl UnsignedIntDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedShort(None) => XSD_UNSIGNED_SHORT,
			Self::UnsignedShort(Some(t)) => t.iri(),
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::UnsignedShort(None) => ParseRdf::parse_rdf(value)
				.map(Value::UnsignedShort)
				.map_err(|_| ParseError),
			Self::UnsignedShort(Some(t)) => t.parse(value),
		}
	}
}

impl_from!(UnsignedIntDatatype {
	ty: UnsignedShortDatatype => Self::UnsignedShort(ty)
});

pub enum UnsignedShortDatatype {
	UnsignedByte,
}

impl UnsignedShortDatatype {
	pub fn iri(&self) -> &'static Iri {
		match self {
			Self::UnsignedByte => XSD_UNSIGNED_BYTE,
		}
	}

	pub fn parse(&self, value: &str) -> Result<Value, ParseError> {
		match self {
			Self::UnsignedByte => ParseRdf::parse_rdf(value)
				.map(Value::UnsignedByte)
				.map_err(|_| ParseError),
		}
	}
}
