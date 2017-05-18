/// A code generator inspired by JavaPoet (https://github.com/square/javapoet)

mod _type;
mod annotation_spec;
mod argument_spec;
mod block;
mod class_spec;
mod constructor_spec;
mod field_spec;
mod imports;
mod interface_spec;
mod method_spec;
mod modifier;
mod section;
mod statement;
mod variable;
mod element_spec;
mod file_spec;

pub use self::_type::*;
pub use self::annotation_spec::*;
pub use self::argument_spec::*;
pub use self::block::*;
pub use self::class_spec::*;
pub use self::constructor_spec::*;
pub use self::field_spec::*;
pub use self::imports::*;
pub use self::interface_spec::*;
pub use self::method_spec::*;
pub use self::modifier::*;
pub use self::section::*;
pub use self::statement::*;
pub use self::variable::*;
pub use self::element_spec::*;
pub use self::file_spec::*;

/// Build modifier lists.
#[macro_export]
macro_rules! mods {
    ($($modifier:expr),*) => {
        {
            let mut tmp_modifiers = Modifiers::new();

            $(
                tmp_modifiers.insert($modifier);
            )*

            tmp_modifiers
        }
    }
}

/// Tool to build statements.
#[macro_export]
macro_rules! stmt {
    ($($var:expr),*) => {{
        let mut statement = Statement::new();
        $(statement.push($var);)*
        statement
    }};
}

#[derive(Debug, Clone)]
pub struct MethodArgument {
    pub modifiers: Modifiers,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_java() {
        let string_type = Type::class("java.lang", "String");
        let list_type = Type::class("java.util", "List");
        let json_creator_type = Type::class("com.fasterxml.jackson.annotation", "JsonCreator");
        let list_of_strings = list_type.with_arguments(vec![&string_type]);

        let values_field = FieldSpec::new(mods![Modifier::Private, Modifier::Final],
                                          &list_of_strings,
                                          "values");

        let values_argument = ArgumentSpec::new(mods![Modifier::Final], &list_of_strings, "values");

        let mut constructor = ConstructorSpec::new(mods![Modifier::Public]);
        constructor.push_annotation(AnnotationSpec::new(json_creator_type));
        constructor.push_argument(&values_argument);
        constructor.push(stmt!["this.values = ", values_argument]);

        let mut values_getter = MethodSpec::new(mods![Modifier::Public], "getValues");
        values_getter.returns(&list_of_strings);
        values_getter.push(stmt!["return this.", &values_field]);

        let mut class = ClassSpec::new(mods![Modifier::Public], "Test");
        class.push_field(&values_field);
        class.push_constructor(&constructor);
        class.push_method(&values_getter);

        let mut file = FileSpec::new("se.tedro");
        file.push_class(&class);

        let result = file.format();

        let reference = ::std::str::from_utf8(include_bytes!("tests/Test.java")).unwrap();
        assert_eq!(reference, result);
    }
}
