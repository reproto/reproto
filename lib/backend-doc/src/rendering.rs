use self::cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use crate::core::errors::*;
use crate::doc_builder::DocBuilder;
use pulldown_cmark as cmark;
use syntect::easy::HighlightLines;
use syntect::highlighting::Theme;
use syntect::html::{
    start_highlighted_html_snippet, styled_line_to_highlighted_html, IncludeBackground,
};
use syntect::parsing::SyntaxSet;

pub fn markdown_to_html(
    out: &mut DocBuilder,
    content: &str,
    theme: &Theme,
    syntax_set: &SyntaxSet,
) -> Result<()> {
    let mut highlighter: Option<HighlightLines> = None;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(content, opts).map(|event| match event {
        Event::Text(text) => {
            if let Some(ref mut highlighter) = highlighter {
                let highlighted = &highlighter.highlight(&text, syntax_set);
                let html = styled_line_to_highlighted_html(highlighted, IncludeBackground::Yes);
                return Event::Html(html.into());
            }

            Event::Text(text)
        }
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref info))) => {
            let syntax = info
                .split(' ')
                .next()
                .and_then(|lang| syntax_set.find_syntax_by_token(lang))
                .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

            highlighter = Some(HighlightLines::new(syntax, theme));

            let (snippet, _) = start_highlighted_html_snippet(theme);
            Event::Html(format!("<div class=\"code\">{}", snippet).into())
        }
        Event::End(Tag::CodeBlock(_)) => {
            highlighter = None;
            Event::Html("</pre></div>".into())
        }
        _ => event,
    });

    let mut buffer = String::new();
    cmark::html::push_html(&mut buffer, parser);
    out.write_str(buffer.as_str())?;
    Ok(())
}
