#[allow(dead_code)]
mod terminal;

use terminal::font;

fn main() {
    let font: terminal::Font = font::FontBuilder::new()
        .font_color(font::Color::TwentyFourBit(122, 122, 0))
        .styles(vec![
            font::Style::Bold,
            font::Style::Strikethrough,
            font::Style::Inverse,
            font::Style::Underline,
        ])
        .build();

    print!("\n{}Custom Font\n\n", font)
}
