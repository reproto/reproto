use super::*;

pub fn is_defined<S>(stmt: S) -> Statement
where
    S: Into<Statement>,
{
    let s = stmt.into();
    stmt![&s, " !== null && ", &s, " !== undefined"]
}

pub fn is_not_defined<S>(stmt: S) -> Statement
where
    S: Into<Statement>,
{
    let s = stmt.into();
    stmt![&s, " === null || ", &s, " === undefined"]
}

pub fn string<S>(s: S) -> Variable
where
    S: ToString,
{
    Variable::String(s.to_string())
}

#[macro_export]
macro_rules! js {
    ([ $arguments:expr ]) => {{
        stmt!["[", $arguments.join(", "), "]"]
    }};

    (= $key:expr, $value:expr) => {{
        stmt![$key, " = ", $value, ";"]
    }};

    (throw $($args:expr),*) => {{
        stmt!["throw new Error(", $($args,)* ");"]
    }};

    (@return $($tail:tt)*) => {{
        stmt!["return ", js![$( $tail )*], ";"]
    }};

    (return $($args:expr),*) => {{
        stmt!["return ", $($args,)* ";"]
    }};

    (new $type:expr, $arguments:expr) => {{
        stmt!["new ", $type, "(", $arguments.join(", "), ")"]
    }};

    (const $name:expr, $($args:expr),*) => {{
        stmt!["const ", $name, " = ", $($args,)* ""]
    }};

    (if $cond:expr, $true:expr) => {{
        let mut el = Elements::new();

        el.push(stmt!["if (", $cond, ") {"]);
        el.push_nested($true);
        el.push("}");

        el
    }};

    (if $cond:expr, $true:expr, $false:expr) => {{
        let mut el = Elements::new();

        el.push(stmt!["if (", $cond, ") {"]);
        el.push_nested($true);
        el.push("} else {");
        el.push_nested($false);
        el.push("}");

        el
    }};

    (for $init:expr; $while:expr; $next:expr, $($body:expr),*) => {{
        let mut el = Elements::new();

        el.push(stmt!["for (", $init, "; ", $while, "; ", $next, ") {"]);
        $(el.push_nested($body.join(Spacing));)*
        el.push("}");

        el
    }}
}
