mod ast;
mod parser;

use errors::*;

#[cfg(test)]
mod tests {
    use super::*;
    use super::ast::*;

    #[test]
    fn message_test() {
        let m = parser::parse_MessageDecl("
        message Foo {
          required string a = 1;
          optional string b = 2;
        }
        ").unwrap();

        assert_eq!("Foo", m.name);
        assert_eq!(2, m.members.len());

        assert_eq!(
            MessageMember::Field(Field::new(Modifier::Required, "a".to_owned(), Type::String, 1)),
            m.members[0]
        );

        assert_eq!(
            MessageMember::Field(Field::new(Modifier::Optional, "b".to_owned(), Type::String, 2)),
            m.members[1]
        );
    }

    #[test]
    fn file_test() {
        let f = parser::parse_File("
        package proto.v1;

        message Foo {
          required string a = 1;
          optional string b = 2;
        }

        interface Bar {
        }
        ").unwrap();

        assert_eq!(Package::new(vec!["proto".to_owned(), "v1".to_owned()]), f.package);
    }
}
