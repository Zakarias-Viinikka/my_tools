//use syntect::easy::HighlightLines;
use syntect::highlighting::{/*Style,*/ ThemeSet};
use syntect::parsing::SyntaxSet;
//use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};
/*
pub fn prettify_text(text: String) -> String {
    let mut pretty_text = "".to_string();

    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let s = text; // THIS IS THE TXT
    for line in LinesWithEndings::from(&s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        pretty_text.push_str(&escaped);
    }

    pretty_text
}*/

use syntect::html::highlighted_html_for_string;

pub fn prettify_text(text: String) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("rs").unwrap();

    highlighted_html_for_string(&text, &ps, syntax, &ts.themes["base16-ocean.dark"]).unwrap()
}
