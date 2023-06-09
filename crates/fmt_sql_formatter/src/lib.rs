use sqlparser::{ast::Statement, parser::Parser};

struct Formatter {
    options: FormatterOptions,
    code: Vec<u8>,
    indentation: u8,
    context: FormatContext,
}

impl Formatter {
    fn into_code(self) -> String {
        unsafe { String::from_utf8_unchecked(self.code) }
    }
}

struct FormatterOptions {
    line_length: usize,
    indentation_kind: IntentationKind,
}

impl Default for FormatterOptions {
    fn default() -> Self {
        Self {
            line_length: 88,
            indentation_kind: IntentationKind::Indent,
        }
    }
}

#[derive(Default)]
struct FormatContext {
    needs_semicolon: bool,
}

enum IntentationKind {
    Indent,
    Space,
}

pub fn format(s: &str) -> String {
    let node = match Parser::new(&sqlparser::dialect::MsSqlDialect {}).try_with_sql(s) {
        Ok(mut parser) => parser.parse_statements().expect("parse sql statements"),
        Err(_) => panic!("failed to parse sql ast"),
    };

    format_node(node, FormatterOptions::default())
}

// TODO(cnpryer): Use `Node`
fn format_node(root: Vec<Statement>, formatter_options: FormatterOptions) -> String {
    let formatter = Formatter {
        options: formatter_options,
        code: Vec::new(),
        indentation: 0,
        context: FormatContext::default(),
    };

    formatter.into_code()
}
