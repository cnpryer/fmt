use fmt_sql_formatter::format;

fn main() {
    println!("Welcome to fmt");
    let s = "SELECT * FROM [fmt]";
    println!("formatting...\n{}", format(s));
}
