use self::cmark::{Event, Options, Parser, Tag, OPTION_ENABLE_FOOTNOTES, OPTION_ENABLE_TABLES};
use backend::errors::*;
use doc_builder::DocBuilder;
use pulldown_cmark as cmark;
use std::borrow::Cow::{Borrowed, Owned};
use syntect::easy::HighlightLines;
use syntect::highlighting::Theme;
use syntect::html::{start_coloured_html_snippet, styles_to_coloured_html, IncludeBackground};
use syntect::parsing::SyntaxSet;

pub fn markdown_to_html(
    out: &mut DocBuilder,
    content: &str,
    theme: &Theme,
    syntax_set: &SyntaxSet,
) -> Result<()> {
    let mut highlighter: Option<HighlightLines> = None;

    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    opts.insert(OPTION_ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(content, opts).map(|event| match event {
        Event::Text(text) => {
            if let Some(ref mut highlighter) = highlighter {
                let highlighted = &highlighter.highlight(&text);
                let html = styles_to_coloured_html(highlighted, IncludeBackground::Yes);
                return Event::Html(Owned(html));
            }

            Event::Text(text)
        }
        Event::Start(Tag::CodeBlock(ref info)) => {
            let syntax = info.split(' ')
                .next()
                .and_then(|lang| syntax_set.find_syntax_by_token(lang))
                .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

            highlighter = Some(HighlightLines::new(syntax, theme));

            let snippet = start_coloured_html_snippet(theme);
            Event::Html(Owned(format!("<div class=\"code\">{}", snippet)))
        }
        Event::End(Tag::CodeBlock(_)) => {
            highlighter = None;
            Event::Html(Borrowed("</pre></div>"))
        }
        _ => event,
    });

    let mut buffer = String::new();
    cmark::html::push_html(&mut buffer, parser);
    out.write_str(buffer.as_str())?;
    Ok(())
}
