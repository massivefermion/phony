// Copyright (C) 2017 1aim GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use quick_xml as xml;
use thiserror::Error;

/// Metadata loading errors.
#[derive(Error, Clone, Debug)]
pub enum Metadata {
    /// EOF was reached before the parsing was complete.
    #[error("unexpected end of file")]
    UnexpectedEof,

    /// A mismatched tag was met.
    #[error("mismatched tag: {0:?}")]
    MismatchedTag(String),

    /// An element was not handled.
    #[error("{phase}: unhandled element: {name:?}")]
    UnhandledElement { phase: String, name: String },

    /// An attribute was not handled.
    #[error("{phase}: unhandled attribute: {name:?}={value:?}")]
    UnhandledAttribute {
        phase: String,
        name: String,
        value: String,
    },
    /// An event was not handled.
    #[error("{phase}: unhandled event: {event:?}")]
    UnhandledEvent { phase: String, event: String },
}

/// Loading of Database) Error
#[derive(Error, Debug)]
pub enum LoadMetadata {
    /// Parsing XML failed, the XML is malformed.
    #[error("Malformed Metadata XML: {0}")]
    Xml(#[from] xml::Error),

    /// Parsing UTF-8 string from XML failed.
    #[error("Non UTF-8 string in Metadata XML: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// Metadata Error
    #[error("{0}")]
    Metadata(#[from] Metadata),

    /// Malformed integer in Metadata XML database
    #[error("Malformed integer in Metadata XML: {0}")]
    Integer(#[from] std::num::ParseIntError),

    /// Malformed boolean in Metadata XML database
    #[error("Malformed boolean in Metadata XML: {0}")]
    Bool(#[from] std::str::ParseBoolError),

    /// I/O-Error while reading Metadata XML database
    #[error("I/O-Error in Metadata XML: {0}")]
    Io(#[from] std::io::Error),
}
