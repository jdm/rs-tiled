use std::{fmt, path::PathBuf};

/// Errors which occured when parsing the file
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A attribute was missing, had the wrong type of wasn't formated
    /// correctly.
    MalformedAttributes(String),
    /// An error occured when decompressing using the
    /// [flate2](https://github.com/alexcrichton/flate2-rs) crate.
    DecompressingError(std::io::Error),
    /// An error occured when decoding a base64 encoded dataset.
    Base64DecodingError(base64::DecodeError),
    /// An error occured when parsing a XML file, such as a TMX or TSX file.
    XmlDecodingError(xml::reader::Error),
    /// The XML stream ended before the document was fully parsed.
    PrematureEnd(String),
    /// The path given is invalid because it isn't contained in any folder.
    PathIsNotFile,
    /// Could not open some file due to an I/O error.
    CouldNotOpenFile {
        /// The path to the file that was unable to be opened.
        path: PathBuf,
        /// The error that occured when trying to open the file.
        err: std::io::Error,
    },
    /// There was an invalid tile in the map parsed.
    InvalidTileFound,
    /// Unknown encoding or compression format or invalid combination of both (for tile layers)
    InvalidEncodingFormat {
        /// The `encoding` attribute of the tile layer data, if any.
        encoding: Option<String>,
        /// The `compression` attribute of the tile layer data, if any.
        compression: Option<String>,
    },
    /// There was an error parsing the value of a [`PropertyValue`].
    ///
    /// [`PropertyValue`]: crate::PropertyValue
    InvalidPropertyValue {
        /// A description of the error that occured.
        description: String,
    },
    /// Found an unknown property value type while parsing a [`PropertyValue`].
    ///
    /// [`PropertyValue`]: crate::PropertyValue
    UnknownPropertyType {
        /// The name of the type that isn't recognized by the crate.
        /// Supported types are `string`, `int`, `float`, `bool`, `color`, `file` and `object`.
        type_name: String,
    },
    /// Found a WangId that was not properly formatted.
    InvalidWangIdEncoding {
        /// Stores the wrongly parsed String.
        read_string: String,
    },
}

/// A result with an error variant of [`crate::Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Error::MalformedAttributes(s) => write!(fmt, "{}", s),
            Error::DecompressingError(e) => write!(fmt, "{}", e),
            Error::Base64DecodingError(e) => write!(fmt, "{}", e),
            Error::XmlDecodingError(e) => write!(fmt, "{}", e),
            Error::PrematureEnd(e) => write!(fmt, "{}", e),
            Error::PathIsNotFile => {
                write!(
                    fmt,
                    "The path given is invalid because it isn't contained in any folder."
                )
            }
            Error::CouldNotOpenFile { path, err } => {
                write!(
                    fmt,
                    "Could not open '{}'. Error: {}",
                    path.to_string_lossy(),
                    err
                )
            }
            Error::InvalidTileFound => write!(fmt, "Invalid tile found in map being parsed"),
            Error::InvalidEncodingFormat { encoding: None, compression: None } =>
                write!(
                    fmt,
                    "Deprecated combination of encoding and compression"
                ),
            Error::InvalidEncodingFormat { encoding, compression } =>
                write!(
                    fmt,
                    "Unknown encoding or compression format or invalid combination of both (for tile layers): {} encoding with {} compression",
                    encoding.as_deref().unwrap_or("no"),
                    compression.as_deref().unwrap_or("no")
                ),
            Error::InvalidPropertyValue{description} =>
                write!(fmt, "Invalid property value: {}", description),
            Error::UnknownPropertyType { type_name } =>
                write!(fmt, "Unknown property value type '{}'", type_name),
            Error::InvalidWangIdEncoding{read_string} =>
                write!(fmt, "\"{}\" is not a valid WangId format", read_string),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DecompressingError(e) => Some(e as &dyn std::error::Error),
            Error::Base64DecodingError(e) => Some(e as &dyn std::error::Error),
            Error::XmlDecodingError(e) => Some(e as &dyn std::error::Error),
            Error::CouldNotOpenFile { err, .. } => Some(err as &dyn std::error::Error),
            _ => None,
        }
    }
}
