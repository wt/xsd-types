use crate::{Datatype, ParseRdf, XsdDatatype};

pub type AnyUri = iref::Uri;

impl XsdDatatype for AnyUri {
	fn type_(&self) -> Datatype {
		Datatype::AnyUri
	}
}

pub type AnyUriBuf = iref::UriBuf;

impl XsdDatatype for AnyUriBuf {
	fn type_(&self) -> Datatype {
		Datatype::AnyUri
	}
}

impl ParseRdf for AnyUriBuf {
	type LexicalForm = AnyUri;
}
