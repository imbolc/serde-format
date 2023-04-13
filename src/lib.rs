//! # serde-format
//!
//! A tiny trait to format a serializable struct using custom placeholders.
//!
//! ## Goals
//!
//! - Be as lightweight as possible
//! - Have no dependencies other than [serde] and [serde_json]
//!
//! ## Non-goals
//!
//! - Prioritize performance
//! - Support any syntax beyond variable substitution
//!
//! ## Usage
//!
//! ```
//! use serde::Serialize;
//! use serde_format::Format;
//!
//! #[derive(Serialize)]
//! struct Foo {
//!     name: String
//! }
//!
//! impl Format for Foo {}
//!
//! let foo = Foo { name: "Bar".into() };
//! assert_eq!(foo.format("Hey, {{name}}!").unwrap(), "Hey, Bar!");
//! ```
//!
//! ## TODO
//!
//! - [ ] A derive macro

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use displaydoc::Display;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

/// Error
#[derive(Debug, Display, Error)]
pub enum Error {
    /// absent variable `{0}`
    NoVar(String),
    /// serde_json
    SerdeJson(#[from] serde_json::Error),
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;

/// A simple formatter with customizable placeholders
pub trait Format {
    /// Left and right placeholders for variables
    const PLACEHOLDERS: (&'static str, &'static str) = ("{{", "}}");

    /// Formats the struct using the template
    fn format(&self, template: impl Into<String>) -> Result<String>
    where
        Self: Serialize,
    {
        let mut result = template.into();
        let data_map: HashMap<String, Value> = serde_json::from_value(serde_json::to_value(self)?)?;
        let (left, right) = Self::PLACEHOLDERS;
        for (key, value) in data_map.iter() {
            let placeholder = format!("{left}{key}{right}");
            if !result.contains(&placeholder) {
                return Err(Error::NoVar(key.into()));
            }
            result = result.replace(
                &placeholder,
                &value
                    .as_str()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| value.to_string()),
            );
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_placeholders() {
        #[derive(Serialize)]
        struct Foo {
            s: String,
            n: u32,
            b: bool,
        }

        impl Format for Foo {}

        let foo = Foo {
            s: "hey".into(),
            n: 1,
            b: true,
        };

        assert_eq!(
            foo.format("s={{s}} n={{n}} b={{b}}").unwrap(),
            "s=hey n=1 b=true"
        );
        assert_eq!(
            foo.format("s={{s}} n={{n}}").err().unwrap().to_string(),
            "absent variable `b`"
        );
    }

    #[test]
    fn custom_placeholders() {
        #[derive(Serialize)]
        struct Foo {
            s: String,
            n: u32,
            b: bool,
        }

        impl Format for Foo {
            const PLACEHOLDERS: (&'static str, &'static str) = ("${", "}");
        }

        let foo = Foo {
            s: "hey".into(),
            n: 1,
            b: true,
        };

        assert_eq!(
            foo.format("s=${s} n=${n} b=${b}").unwrap(),
            "s=hey n=1 b=true"
        );
        assert_eq!(
            foo.format("s=${s} n=${n}").err().unwrap().to_string(),
            "absent variable `b`"
        );
    }
}
