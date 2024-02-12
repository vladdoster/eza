use super::file_name::QuoteStyle;
use ansiterm::{ANSIString, Style};

pub fn escape(
    string: String,
    bits: &mut Vec<ANSIString<'_>>,
    good: Style,
    bad: Style,
    quote_style: QuoteStyle,
) {
    let bits_starting_length = bits.len();
    let needs_quotes = string.contains(' ') || string.contains('\'');
    let quote_bit = good.paint(if string.contains('\'') { "\"" } else { "\'" });

    if string
        .chars()
        .all(|c| c >= 0x20 as char && c != 0x7f as char)
    {
        bits.push(good.paint(string));
    } else {
        for c in string.chars() {
            // The `escape_default` method on `char` is *almost* what we want here, but
            // it still escapes non-ASCII UTF-8 characters, which are still printable.

            // TODO: This allocates way too much,
            // hence the `all` check above.
            if c >= 0x20 as char && c != 0x7f as char {
                bits.push(good.paint(c.to_string()));
            } else {
                bits.push(bad.paint(c.escape_default().to_string()));
            }
        }
    }

    if quote_style != QuoteStyle::NoQuotes && needs_quotes {
        bits.insert(bits_starting_length, quote_bit.clone());
        bits.push(quote_bit);
    }
}
