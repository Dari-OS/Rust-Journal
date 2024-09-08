mod styles;
use styles::Styles;
fn main() {
    println!(
        "{}Hello World!{}\nThis is not formated",
        Styles::UNDERLINE | Styles::BOLD,
        Styles::RESET
    );

    styled_print!("Styled by a Macro", Styles::STRIKETHROUGH | Styles::ITALIC)
}
