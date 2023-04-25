//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Declaration.cs>
//!
//! ## Implementation notes
//!
//! [`Range`] has been replaced with the more idiomatic [`RangeInclusive<Position>`].

use antlr_rust::token::Token;
use rusty_yarn_spinner_core::prelude::Value;
use rusty_yarn_spinner_core::types::Type;
use std::cell::Ref;
use std::fmt::Display;
use std::ops::RangeInclusive;

/// Information about a declaration. Stored inside a declaration table,
/// which is produced from the Compiler.
///
/// You do not create instances of this class yourself. They are
/// generated by the [`Compiler`].
#[derive(Debug, Clone)]
pub struct Declaration {
    /// The name of this declaration.
    pub name: String,

    /// The default value of this declaration, if no value has been
    /// specified in code or is available from a [`Dialogue`]'s
    /// [`IVariableStorage`].
    pub default_value: Value,

    /// A string describing the purpose of this declaration.
    pub description: String,

    /// The name of the file in which this declaration was found.
    ///
    /// If this declaration was not found in a Yarn source file, this
    /// will be [`DeclarationSource::External`].
    pub source_file_name: DeclarationSource,

    /// The name of the node in which this declaration was found.
    ///
    /// If this declaration was not found in a Yarn source file, this
    /// will be [`None`].
    pub source_node_name: Option<String>,

    /// The line number at which this declaration was found in the
    /// source file.
    ///
    /// If this declaration was not found in a Yarn source file, this
    /// will be [`None`].
    pub source_file_line: Option<usize>,

    /// A value indicating whether this declaration was implicitly
    /// inferred from usage.
    ///
    /// If `true`, this declaration was implicitly inferred from usage.
    /// If `false`, this declaration appears in the source code.
    pub is_implicit: bool,

    /// The type of the variable, as represented by an object found
    /// in a variant of [`Type`].
    pub r#type: Type,

    /// The range of text at which this declaration occurs.
    ///
    /// This range refers to the declaration of the symbol itself, and
    /// not any syntax surrounding it. For example, the declaration
    /// `<<declare $x = 1>>` would have a range referring to the `$x`
    /// symbol.
    pub range: RangeInclusive<Position>,
}

#[derive(Debug, Clone)]
/// The source of a declaration.
///
/// ## Implementation notes
///
/// In the original implementation, [`External`] is just a magic string.
pub enum DeclarationSource {
    External,
    File(String),
}

/// Represents a position in a multi-line string.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    /// The zero-indexed line of this position.
    pub line: usize,

    /// The zero-indexed character number of this position.
    pub character: usize,
}

impl Position {
    pub fn from_token(token: Ref<impl Token + ?Sized>) -> Self {
        Self {
            line: (token.get_line() - 1) as usize,
            character: (token.get_column()) as usize,
        }
    }
}

impl Display for DeclarationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External => write!(f, "(External)"),
            Self::File(file_name) => write!(f, "{}", file_name),
        }
    }
}
