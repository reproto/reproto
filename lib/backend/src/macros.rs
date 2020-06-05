/// Helper macro to implement listeners opt loop.
#[macro_export]
macro_rules! code {
    ($codes:expr, $context:path) => {{
        let mut t = Tokens::new();

        for c in $codes {
            if let $context { .. } = c.context {
                t.append({
                    let mut t = Tokens::new();

                    for line in &c.lines {
                        t.push(line.as_str());
                    }

                    t
                });
            }
        }

        t
    }};
}

#[macro_export]
macro_rules! code_in {
    ($receiver:expr, $codes:expr, $context:path) => {{
        for c in $codes {
            if let $context { .. } = c.context {
                for line in &c.lines {
                    $receiver.push();
                    $receiver.append(line.as_str());
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! code_contains {
    ($codes:expr, $context:path) => {{
        $codes.iter().any(|c| {
            if let $context { .. } = c.context {
                true
            } else {
                false
            }
        })
    }};
}
