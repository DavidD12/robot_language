use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use line_col::LineColLookup;

use crate::parser::Position;

pub enum RlError {
    File {
        filename: String,
        message: String,
    },
    Parse {
        message: String,
        position: Option<Position>,
        expected: Vec<String>,
    },
    Other(String),
}

impl RlError {
    pub fn new_parse(lookup: &LineColLookup, error: ParseError<usize, Token, &str>) -> Self {
        match error {
            ParseError::InvalidToken { location } => Self::Parse {
                message: "invalid token".into(),
                position: Some(Position::new(lookup, location)),
                expected: Vec::new(),
            },
            ParseError::UnrecognizedEOF { location, expected } => Self::Parse {
                message: "unreconized EOF".into(),
                position: Some(Position::new(lookup, location)),
                expected,
            },
            ParseError::UnrecognizedToken { token, expected } => Self::Parse {
                message: format!("unreconized token '{}'", token.1),
                position: Some(Position::new(lookup, token.0)),
                expected,
            },
            ParseError::ExtraToken { token } => Self::Parse {
                message: format!("extra token '{}'", token.1),
                position: Some(Position::new(lookup, token.0)),
                expected: Vec::new(),
            },
            ParseError::User { error } => Self::Parse {
                message: format!("parse error '{}'", error),
                position: None,
                expected: Vec::new(),
            },
        }
    }
}

impl std::fmt::Display for RlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RlError::File { filename, message } => {
                write!(f, "cannot read file {} {}", filename, message)?;
            }
            RlError::Parse {
                message,
                position,
                expected,
            } => match position {
                Some(position) => write!(
                    f,
                    "parse error '{}' at {}, expecting: {:?}",
                    message, position, expected
                )?,
                None => write!(f, "parse error '{}', expecting: {:?}", message, expected)?,
            },
            RlError::Other(msg) => write!(f, "error: {}", msg)?,
        }
        Ok(())
    }
}
