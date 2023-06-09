use fmt_sql_formatter::{format_with_options, FormatterOptions};

fn main() {
    println!("Welcome to fmt");
    let s = r#"
SELECT [col1], [col2], [col3], [col4]
FROM [table1] JOIN [table2] ON ([table1].[id] = [table2].[id])
"#;
    println!(
        "formatting...\n{}",
        format_with_options(s, FormatterOptions::default())
    );
}
