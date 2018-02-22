//! Utility functions for the parser.

use std::borrow::Cow;

/// Check if character is not an indentation character.
fn is_not_indent(c: char) -> bool {
    match c {
        ' ' | '\t' => false,
        _ => true,
    }
}

/// Strip common indent from all input lines.
pub fn strip_code_block<'a>(input: Cow<'a, str>) -> Vec<Cow<'a, str>> {
    let num_empty_start = input
        .lines()
        .take_while(|line| line.chars().all(char::is_whitespace))
        .count();

    let num_empty_end = input
        .lines()
        .rev()
        .take_while(|line| line.chars().all(char::is_whitespace))
        .count();

    let indent = input
        .lines()
        .flat_map(|line| line.find(is_not_indent).into_iter())
        .min();

    match input {
        Cow::Borrowed(input) => {
            let mut it = input.lines();

            // strip empty lines from the front
            for _ in 0..num_empty_start {
                it.next();
            }

            // strip empty lines from the tail
            for _ in 0..num_empty_end {
                it.next_back();
            }

            if let Some(indent) = indent {
                return it.map(|line| {
                    if line.len() >= indent {
                        Cow::Borrowed(&line[indent..])
                    } else {
                        Cow::Borrowed(line)
                    }
                }).collect();
            }

            return it.map(Cow::Borrowed).collect();
        }
        Cow::Owned(input) => {
            let mut it = input.lines();

            // strip empty lines from the front
            for _ in 0..num_empty_start {
                it.next();
            }

            // strip empty lines from the tail
            for _ in 0..num_empty_end {
                it.next_back();
            }

            if let Some(indent) = indent {
                return it.map(|line| {
                    if line.len() >= indent {
                        Cow::Owned(line[indent..].to_string())
                    } else {
                        Cow::Owned(line.to_string())
                    }
                }).collect();
            }

            return it.map(|s| Cow::Owned(s.to_string())).collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_code_block() {
        let result = strip_code_block("\n   hello\n  world\n\n\n again\n\n\n".into());
        let expected: Vec<Cow<'static, str>> = vec!["  hello".into(), " world".into(), "".into(), "".into(), "again".into()];

        assert_eq!(expected, result);
    }
}
