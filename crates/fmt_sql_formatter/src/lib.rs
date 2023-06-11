#![doc = include_str!("../README.md")]

use sqlparser::{ast::Statement, parser::Parser};

const DEFAULT_LINE_LENGTH: usize = 88;
const DEFAULT_INDENTATION_KIND: IntentationKind = IntentationKind::Indent;

#[derive(Default)]
struct Formatter {
    options: FormatterOptions,
    generator: CodeGenerator,
    current_context: FormatContext,
}

impl Formatter {
    /// Get `String` of generated source code.
    pub fn into_code(self) -> String {
        self.generator.into_code()
    }

    /// Create a formatter from `FormatterOptions`.
    pub fn from_options(options: FormatterOptions) -> Formatter {
        Formatter {
            options,
            ..Default::default()
        }
    }

    /// Generate formated `sqlparser::ast::Statement`s.
    pub fn with_statements(mut self, statements: Vec<Statement>) -> Formatter {
        self.generator
            .push_bytes(self.format_statements(statements));
        self
    }

    // TODO(cnpryer)
    fn format_statements(&self, statements: Vec<Statement>) -> Vec<u8> {
        let mut bytes = Vec::new();
        statements.into_iter().for_each(|stmt| {
            let sep = self.next_statement_sep(&stmt);
            bytes.extend(self.format_statement(stmt));
            bytes.extend(sep);
        });
        bytes
    }

    fn format_statement(&self, stmt: Statement) -> Vec<u8> {
        todo!()
    }

    fn next_statement_sep(&self, stmt: &Statement) -> Vec<u8> {
        todo!()
    }
}

/// `CodeGenerator` is used to generate SQL statements as source code.
#[derive(Default)]
struct CodeGenerator {
    bytes: Vec<u8>,
}

impl CodeGenerator {
    /// Push a byte to the generated code.
    fn push(&mut self, b: u8) {
        self.bytes.push(b);
    }

    /// Pushes bytes to the generated code.
    fn push_bytes(&mut self, bytes: Vec<u8>) {
        bytes.into_iter().for_each(|b| self.push(b))
    }

    /// Move into a `String` of the current generated code.
    pub fn into_code(self) -> String {
        unsafe { String::from_utf8_unchecked(self.bytes) }
    }
}

#[derive(PartialEq, Eq)]
pub struct FormatterOptions {
    line_length: usize,
    indentation_kind: IntentationKind,
}

impl Default for FormatterOptions {
    fn default() -> Self {
        Self {
            line_length: DEFAULT_LINE_LENGTH,
            indentation_kind: DEFAULT_INDENTATION_KIND,
        }
    }
}

#[derive(Default)]
struct FormatContext {
    needs_semicolon: bool,
    current_indentation: u8,
}

#[derive(PartialEq, Eq)]
enum IntentationKind {
    Indent,
    Space,
}

// TODO(cnpryer): Allow comments https://github.com/cnpryer/fmt/issues/1
//   Ruff uses CommentRanges to manage formatting code containing comments.
pub fn format(s: &str) -> String {
    let statements = parse_statements(s);
    format_statements(statements, FormatterOptions::default())
}

pub fn format_with_options(s: &str, options: FormatterOptions) -> String {
    let statements = parse_statements(s);
    format_statements(statements, options)
}

fn parse_statements(s: &str) -> Vec<Statement> {
    match Parser::new(&sqlparser::dialect::MsSqlDialect {}).try_with_sql(s) {
        Ok(mut parser) => parser.parse_statements().expect("parse sql statements"),
        Err(_) => panic!("failed to parse sql ast"),
    }
}

fn format_statements(statements: Vec<Statement>, formatter_options: FormatterOptions) -> String {
    Formatter::from_options(formatter_options)
        .with_statements(statements)
        .into_code()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_one_liner() {
        let s = "select top 10 * from table";
        let expected = r"SELECT TOP 10 * FROM [table]";
        assert_eq!(format(s), expected);
    }

    fn test_basic() {
        let s = "select a, b, c, d from table1 join table2 on table1.id = table2.id and table1.other_id = table2.other_id where table1.name = 'something'";
        let expected = r#"SELECT
    [a],
    [b],
    [c],
    [d]
FROM
    [table1]
    JOIN [table2] ON (
        [table1].[id] = [table1].[id]
        AND [table1].[other_id] = [table2].[other_id]
    )
WHERE
    [table1].[name] = 'something'"#;
        assert_eq!(format(s), expected);
    }
}
