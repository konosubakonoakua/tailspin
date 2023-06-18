use crate::color::{Bg, Fg, Style};
use crate::highlighters::quotes::State::{InsideQuote, OutsideQuote};
use crate::highlighters::HighlightFn;

pub fn highlight(fg: Fg, bg: Bg, style: Style, quotes_token: char) -> HighlightFn {
    let color = crate::color::to_ansi(fg, bg, style);

    Box::new(move |input: &str| -> String { highlight_inside_quotes(&color, input, quotes_token) })
}

enum State {
    InsideQuote {
        color_inside_quote: String,
        potential_reset_code: String,
    },
    OutsideQuote,
}

fn highlight_inside_quotes(color: &str, input: &str, quotes_token: char) -> String {
    const RESET: &str = "\x1b[0m";

    let has_unmatched_quotes = input.chars().filter(|&ch| ch == quotes_token).count() % 2 != 0;
    if has_unmatched_quotes {
        return input.to_string();
    }

    let mut state = OutsideQuote;
    let mut output = String::new();

    for ch in input.chars() {
        match &mut state {
            InsideQuote {
                color_inside_quote: color,
                ref mut potential_reset_code,
            } => {
                if ch == quotes_token {
                    output.push(ch);
                    output.push_str(RESET);
                    state = OutsideQuote;
                    continue;
                }

                potential_reset_code.push(ch);
                if potential_reset_code.as_str() == RESET {
                    output.push_str(potential_reset_code);
                    output.push_str(color);
                    potential_reset_code.clear();
                } else if !RESET.starts_with(potential_reset_code.as_str()) {
                    output.push_str(potential_reset_code);
                    potential_reset_code.clear();
                }
            }
            OutsideQuote => {
                if ch == quotes_token {
                    output.push_str(color);
                    output.push(ch);
                    state = InsideQuote {
                        color_inside_quote: color.to_string(),
                        potential_reset_code: String::new(),
                    };
                    continue;
                }

                output.push(ch);
            }
        };
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight_quotes_with_ansi() {
        let highlighter = highlight(Fg::Red, Bg::None, Style::None, '"');
        let result = highlighter("outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside");
        let expected =
            "outside \x1b[33\"hello \x1b[34;42;3m42\x1b[0m\x1b[33 world\"\x1b[0m outside";
        assert_eq!(result, expected);
    }

    #[test]
    fn highlight_quotes_without_ansi() {
        let color = "[color]";
        let input = "outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside";
        let result = highlight_inside_quotes(color, input, '"');
        let expected =
            "outside [color]\"hello \x1b[34;42;3m42\x1b[0m[color] world\"\x1b[0m outside";
        assert_eq!(result, expected);
    }

    #[test]
    fn do_nothing_on_uneven_number_of_quotes() {
        let highlighter = highlight(Fg::Red, Bg::None, Style::None, '"');
        let result = highlighter("outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside");
        let expected = "outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside";
        assert_eq!(result, expected);
    }
}
