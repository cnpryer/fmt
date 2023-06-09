WIP: Create formatted SQL.

```rust
use fmt_sql_formatter::{format, FormatOptions};

const s: &str = "SELECT * FROM [table]";
println("{}", format(s, FormatOptions::default()));
```

- `Formatter` - A wrapper for a `CodeGenerator` implementation used to format SQL code.
- `CodeGenerator` - A struct used for generating code.