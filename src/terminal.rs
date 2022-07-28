use std::fmt;

pub type Font = String;

pub const ESC_PREFIX: &str = "\x1b[";

#[non_exhaustive]
struct EscapeSequenceCode;

impl EscapeSequenceCode {
    pub const OUTPUT_RESET: u8 = 0;

    pub const FONT_COLOR_BLACK: u8 = 30;
    pub const FONT_COLOR_RED: u8 = 31;
    pub const FONT_COLOR_GREEN: u8 = 32;
    pub const FONT_COLOR_YELLOW: u8 = 33;
    pub const FONT_COLOR_BLUE: u8 = 34;
    pub const FONT_COLOR_MAGENTA: u8 = 35;
    pub const FONT_COLOR_CYAN: u8 = 36;
    pub const FONT_COLOR_WHITE: u8 = 37;

    pub const FONT_COLOR_BRIGHT_BLACK: u8 = 90;
    pub const FONT_COLOR_BRIGHT_RED: u8 = 91;
    pub const FONT_COLOR_BRIGHT_GREEN: u8 = 92;
    pub const FONT_COLOR_BRIGHT_YELLOW: u8 = 93;
    pub const FONT_COLOR_BRIGHT_BLUE: u8 = 94;
    pub const FONT_COLOR_BRIGHT_MAGENTA: u8 = 95;
    pub const FONT_COLOR_BRIGHT_CYAN: u8 = 96;
    pub const FONT_COLOR_BRIGHT_WHITE: u8 = 97;

    pub const FONT_COLOR_DEFAULT: u8 = 39;

    pub const FONT_BG_COLOR_BLACK: u8 = 40;
    pub const FONT_BG_COLOR_RED: u8 = 41;
    pub const FONT_BG_COLOR_GREEN: u8 = 42;
    pub const FONT_BG_COLOR_YELLOW: u8 = 43;
    pub const FONT_BG_COLOR_BLUE: u8 = 44;
    pub const FONT_BG_COLOR_MAGENTA: u8 = 45;
    pub const FONT_BG_COLOR_CYAN: u8 = 46;
    pub const FONT_BG_COLOR_WHITE: u8 = 47;

    pub const FONT_BG_COLOR_BRIGHT_BLACK: u8 = 100;
    pub const FONT_BG_COLOR_BRIGHT_RED: u8 = 101;
    pub const FONT_BG_COLOR_BRIGHT_GREEN: u8 = 102;
    pub const FONT_BG_COLOR_BRIGHT_YELLOW: u8 = 103;
    pub const FONT_BG_COLOR_BRIGHT_BLUE: u8 = 104;
    pub const FONT_BG_COLOR_BRIGHT_MAGENTA: u8 = 105;
    pub const FONT_BG_COLOR_BRIGHT_CYAN: u8 = 106;
    pub const FONT_BG_COLOR_BRIGHT_WHITE: u8 = 107;

    pub const FONT_BG_COLOR_DEFAULT: u8 = 49;
}

pub trait EscapeSequence: fmt::Display {
    fn escape_sequence(&self) -> String;
}

pub mod font {
    use std::fmt;

    use super::{EscapeSequence, EscapeSequenceCode, ESC_PREFIX, Font};

    const EIGHT_BIT_FONT_COLOR_PREFIX: &str = "38;5;";
    const TWENTY_FOUR_BIT_FONT_COLOR_PREFIX: &str = "38;2;";

    const EIGHT_BIT_FONT_BG_COLOR_PREFIX: &str = "48;5;";
    const TWENTY_FOUR_BIT_FONT_BG_COLOR_PREFIX: &str = "48;2;";

    pub enum Color {
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
        Default,
        BrightBlack,
        BrightRed,
        BrightGreen,
        BrightYellow,
        BrightBlue,
        BrightMagenta,
        BrightCyan,
        BrightWhite,
        EightBit(u8),
        TwentyFourBit(u8, u8, u8),
    }

    impl Default for Color {
        fn default() -> Self {
            Color::Default
        }
    }

    impl EscapeSequence for Color {
        fn escape_sequence(&self) -> String {
            let mut escape_sequence = String::from(ESC_PREFIX);

            let mut add_escape_sequence_code = |escape_sequence_code: u8| {
                escape_sequence.push_str(&escape_sequence_code.to_string())
            };

            match self {
                Color::Black => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BLACK),
                Color::Red => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_RED),
                Color::Green => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_GREEN),
                Color::Yellow => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_YELLOW),
                Color::Blue => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BLUE),
                Color::Magenta => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_MAGENTA),
                Color::Cyan => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_CYAN),
                Color::White => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_WHITE),

                Color::Default => add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_DEFAULT),

                Color::BrightBlack => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_BLACK)
                }
                Color::BrightRed => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_RED)
                }
                Color::BrightGreen => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_GREEN)
                }
                Color::BrightYellow => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_YELLOW)
                }
                Color::BrightBlue => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_BLUE)
                }
                Color::BrightMagenta => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_MAGENTA)
                }
                Color::BrightCyan => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_CYAN)
                }
                Color::BrightWhite => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_COLOR_BRIGHT_WHITE)
                }

                Color::EightBit(value) => {
                    escape_sequence.push_str(EIGHT_BIT_FONT_COLOR_PREFIX);
                    escape_sequence.push_str(&value.to_string())
                }

                Color::TwentyFourBit(r, g, b) => {
                    escape_sequence.push_str(TWENTY_FOUR_BIT_FONT_COLOR_PREFIX);
                    escape_sequence.push_str(&format!("{};{};{}", r, g, b))
                }
            }

            escape_sequence.push('m');

            escape_sequence
        }
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.escape_sequence())
        }
    }

    pub enum BackgroundColor {
        Black,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        White,
        Default,
        BrightBlack,
        BrightRed,
        BrightGreen,
        BrightYellow,
        BrightBlue,
        BrightMagenta,
        BrightCyan,
        BrightWhite,
        EightBit(u8),
        TwentyFourBit(u8, u8, u8),
    }

    impl Default for BackgroundColor {
        fn default() -> Self {
            BackgroundColor::Default
        }
    }

    impl EscapeSequence for BackgroundColor {
        fn escape_sequence(&self) -> String {
            let mut escape_sequence = String::from(ESC_PREFIX);

            let mut add_escape_sequence_code = |escape_sequence_code: u8| {
                escape_sequence.push_str(&escape_sequence_code.to_string())
            };

            match self {
                BackgroundColor::Black => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BLACK)
                }
                BackgroundColor::Red => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_RED)
                }
                BackgroundColor::Green => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_GREEN)
                }
                BackgroundColor::Yellow => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_YELLOW)
                }
                BackgroundColor::Blue => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BLUE)
                }
                BackgroundColor::Magenta => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_MAGENTA)
                }
                BackgroundColor::Cyan => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_CYAN)
                }
                BackgroundColor::White => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_WHITE)
                }

                BackgroundColor::Default => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_DEFAULT)
                }

                BackgroundColor::BrightBlack => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_BLACK)
                }
                BackgroundColor::BrightRed => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_RED)
                }
                BackgroundColor::BrightGreen => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_GREEN)
                }
                BackgroundColor::BrightYellow => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_YELLOW)
                }
                BackgroundColor::BrightBlue => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_BLUE)
                }
                BackgroundColor::BrightMagenta => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_MAGENTA)
                }
                BackgroundColor::BrightCyan => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_CYAN)
                }
                BackgroundColor::BrightWhite => {
                    add_escape_sequence_code(EscapeSequenceCode::FONT_BG_COLOR_BRIGHT_WHITE)
                }

                BackgroundColor::EightBit(value) => {
                    escape_sequence.push_str(EIGHT_BIT_FONT_BG_COLOR_PREFIX);
                    escape_sequence.push_str(&value.to_string())
                }

                BackgroundColor::TwentyFourBit(r, g, b) => {
                    escape_sequence.push_str(TWENTY_FOUR_BIT_FONT_BG_COLOR_PREFIX);
                    escape_sequence.push_str(&format!("{};{};{}", r, g, b))
                }
            }

            escape_sequence.push('m');

            escape_sequence
        }
    }

    impl fmt::Display for BackgroundColor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.escape_sequence())
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum Style {
        Default = -1,
        Bold = 1,
        Dim = 2,
        Italic = 3,
        Underline = 4,
        Blinking = 5,
        Inverse = 7,
        Hidden = 8,
        Strikethrough = 9,
    }

    impl Default for Style {
        fn default() -> Self {
            Style::Default
        }
    }

    impl EscapeSequence for Style {
        fn escape_sequence(&self) -> String {
            let mut escape_sequence = String::from(ESC_PREFIX);
            
            escape_sequence.push_str(&(*self as u8).to_string());

            escape_sequence.push('m');

            escape_sequence
        }
    }

    impl fmt::Display for Style {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.escape_sequence())
        }
    }

    #[derive(Default)]
    pub struct FontBuilder {
        background_color: BackgroundColor,
        font_styles: Vec<Style>,
        font_color: Color,
    }

    impl FontBuilder {
        pub fn new() -> Self {
            FontBuilder::default()
        }

        pub fn background_color(mut self, background_color: BackgroundColor) -> Self {
            self.background_color = background_color;
            self
        }

        pub fn style(mut self, style: Style) -> Self {
            if !self.font_styles.contains(&style) {
                self.font_styles.push(style);
            }

            self
        }

        pub fn styles(mut self, styles: Vec<Style>) -> Self {
            styles.iter().for_each(| style | {
                if !self.font_styles.contains(style) {
                    self.font_styles.push(*style);
                }
            });

            self
        }

        pub fn font_color(mut self, font_color: Color) -> Self {
            self.font_color = font_color;
            self
        }

        pub fn build(&self) -> Font {
            let mut font = String::from(self.background_color.to_string());

            self.font_styles.iter().for_each(| style | {
                font.push_str(&style.to_string())
            });

            font.push_str(&self.font_color.to_string());

            font
        }

    }


}
