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
//! assert_eq!(foo.format("Hey, {{name}}!"), "Hey, Bar!");
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// A simple formatter with customizable placeholders
pub trait Format {
    /// Left and right placeholders for variables
    const PLACEHOLDERS: (&'static str, &'static str) = ("{{", "}}");

    /// Formats the struct using the template
    fn format(&self, template: impl Into<String>) -> String
    where
        Self: Serialize,
    {
        let mut result = template.into();
        let data_map: HashMap<String, Value> =
            serde_json::from_value(serde_json::to_value(self).unwrap_or_default())
                .unwrap_or_default();
        let (left, right) = Self::PLACEHOLDERS;
        for (key, value) in data_map.iter() {
            let placeholder = format!("{left}{key}{right}");
            result = result.replace(
                &placeholder,
                &value
                    .as_str()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| value.to_string()),
            );
        }
        result
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

        assert_eq!(foo.format("s={{s}} n={{n}} b={{b}}"), "s=hey n=1 b=true");
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

        assert_eq!(foo.format("s=${s} n=${n} b=${b}"), "s=hey n=1 b=true");
    }
}
