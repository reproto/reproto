mod argument_spec;
mod class_spec;
mod decorator_spec;
mod element_spec;
mod file_spec;
mod imports;
mod method_spec;
mod name;
mod statement;
mod variable;

pub use self::argument_spec::*;
pub use self::class_spec::*;
pub use self::decorator_spec::*;
pub use self::element_spec::*;
pub use self::file_spec::*;
pub use self::imports::*;
pub use self::method_spec::*;
pub use self::name::*;
pub use self::statement::*;
pub use self::variable::*;

/// Tool to build statements.
#[macro_export]
macro_rules! python_stmt {
    ($($var:expr),*) => {{
        let mut statement = Statement::new();
        $(statement.push($var);)*
        statement
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python() {
        let static_method = Name::built_in("staticmethod");
        let exit = Name::imported("sys", "exit");

        let mut file = FileSpec::new();

        let mut hello = MethodSpec::new("hello");
        hello.push_decorator(static_method);
        hello.push(python_stmt!["return 12"]);

        let mut bye = MethodSpec::new("bye");
        bye.push(python_stmt![exit, "(1)"]);

        let mut foo = ClassSpec::new("Foo");
        foo.push(hello);
        foo.push(bye);

        file.push(foo);

        let result = file.format();

        let reference = ::std::str::from_utf8(include_bytes!("tests/test.py")).unwrap();
        assert_eq!(reference, result);
    }
}
