use codeviz::js::*;

pub fn is_defined<S>(stmt: S) -> Statement
    where S: Into<Statement>
{
    let s = stmt.into();
    stmt![&s, " !== null && ", &s, " !== undefined"]
}

pub fn is_not_defined<S>(stmt: S) -> Statement
    where S: Into<Statement>
{
    let s = stmt.into();
    stmt![&s, " === null || ", &s, " === undefined"]
}

pub fn string<S>(s: S) -> Variable
    where S: ToString
{
    Variable::String(s.to_string())
}

#[macro_export]
macro_rules! if_stmt {
    ($cond:expr, $true:expr) => {{
        let mut if_stmt = Elements::new();

        if_stmt.push(stmt!["if (", $cond, ") {"]);
        if_stmt.push_nested($true);
        if_stmt.push("}");

        if_stmt
    }};

    ($cond:expr, $true:expr, $false:expr) => {{
        let mut if_stmt = Elements::new();

        if_stmt.push(stmt!["if (", $cond, ") {"]);
        if_stmt.push_nested($true);
        if_stmt.push("} else {");
        if_stmt.push_nested($false);
        if_stmt.push("}");

        if_stmt
    }};
}
