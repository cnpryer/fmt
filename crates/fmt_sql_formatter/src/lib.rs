#![doc = include_str!("../README.md")]

use formatter::Formatter;
pub use formatter::FormatterOptions;
use sqlparser::{ast::Statement, parser::Parser};

mod formatter;
mod generator;

// TODO(cnpryer): Allow comments https://github.com/cnpryer/fmt/issues/1
//   Ruff uses CommentRanges to manage formatting code containing comments.
pub fn format(s: &str) -> String {
    let mut formatter = Formatter::new(FormatterOptions::default());
    let statements = parse_statements(s);
    formatter.format(statements);
    formatter.into_code()
}

pub fn format_with_options(s: &str, options: FormatterOptions) -> String {
    let mut formatter = Formatter::new(options);
    let statements = parse_statements(s);
    formatter.format(statements);
    formatter.into_code()
}

fn parse_statements(s: &str) -> Vec<Statement> {
    match Parser::new(&sqlparser::dialect::MsSqlDialect {}).try_with_sql(s) {
        Ok(mut parser) => parser.parse_statements().expect("parse sql statements"),
        Err(_) => panic!("failed to parse sql ast"),
    }
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
