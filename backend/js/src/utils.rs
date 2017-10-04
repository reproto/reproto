pub fn is_defined<'el, S>(toks: S) -> ::genco::Tokens<'el, ::genco::JavaScript<'el>>
where
    S: Into<::genco::Tokens<'el, ::genco::JavaScript<'el>>>,
{
    let s = toks.into();
    toks![s.clone(), " !== null && ", s, " !== undefined"]
}

pub fn is_not_defined<'el, S>(toks: S) -> ::genco::Tokens<'el, ::genco::JavaScript<'el>>
where
    S: Into<::genco::Tokens<'el, ::genco::JavaScript<'el>>>,
{
    let s = toks.into();
    toks![s.clone(), " === null || ", s, " === undefined"]
}

#[macro_export]
macro_rules! js {
    ([ $arguments:expr ]) => {{
        toks!["[", $arguments.join(", "), "]"]
    }};

    (= $key:expr, $value:expr) => {{
        toks![$key, " = ", $value, ";"]
    }};

    (throw $($args:expr),*) => {{
        toks!["throw new Error(", $($args,)* ");"]
    }};

    (@return $($tail:tt)*) => {{
        toks!["return ", js![$( $tail )*], ";"]
    }};

    (return $($args:expr),*) => {{
        toks!["return ", $($args,)* ";"]
    }};

    (new $type:expr, $arguments:expr) => {{
        toks!["new ", $type, "(", $arguments.join(", "), ")"]
    }};

    (if $cond:expr, $true:expr) => {{
        let mut el = Tokens::new();

        el.push(toks!["if (", $cond, ") {"]);
        el.nested($true);
        el.push("}");

        el
    }};

    (if $cond:expr, $true:expr, $false:expr) => {{
        let mut el = Tokens::new();

        el.push(toks!["if (", $cond, ") {"]);
        el.nested($true);
        el.push("} else {");
        el.nested($false);
        el.push("}");

        el
    }};

    (for $init:expr; $while:expr; $next:expr, $($body:expr),*) => {{
        let mut el = Tokens::new();

        el.push(toks!["for (", $init, "; ", $while, "; ", $next, ") {"]);
        $(el.nested($body.join_line_spacing());)*
        el.push("}");

        el
    }}
}
