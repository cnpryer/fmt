WIP: Create formatted SQL.

```rust
use fmt_sql_formatter::{format, FormatOptions};

const s: &str = "SELECT ... FROM [table]";
println("{}", format(s);
```

How:

- `Formatter` - A formatter responsible for the generation of formatted SQL code.
- `CodeGenerator` - A struct used to generate SQL code with.
- `FormatterOptions` - Data used by the `Formatter` to format SQL.
- `FormatContext` - Context the `Formatter` considers when formatting SQL.