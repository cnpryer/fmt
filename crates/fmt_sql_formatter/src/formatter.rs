use sqlparser::ast::Statement;

use crate::generator::CodeGenerator;

const DEFAULT_LINE_LENGTH: u8 = 88;
const DEFAULT_INDENT_KIND: IndentKind = IndentKind::Tab;

/// A formatter used to generate formatted code.
pub(crate) struct Formatter {
    options: FormatterOptions,
    generator: CodeGenerator,
}

impl Formatter {
    pub fn new(options: FormatterOptions) -> Self {
        Self {
            options,
            generator: CodeGenerator::with_capacity(todo!()),
        }
    }

    // TODO(cnpryer): Use `Node`
    pub fn format(&mut self, statements: Vec<Statement>) {
        todo!()
    }

    pub fn into_code(self) -> String {
        self.generator.into_code()
    }
}

pub struct FormatterOptions {
    line_length: u8,
    indent_kind: IndentKind,
}

impl Default for FormatterOptions {
    fn default() -> Self {
        Self {
            line_length: DEFAULT_LINE_LENGTH,
            indent_kind: DEFAULT_INDENT_KIND,
        }
    }
}

enum IndentKind {
    Space,
    Tab,
}
