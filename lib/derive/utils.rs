/// Check if the given input looks like a datetime type.
pub fn is_datetime(input: &str) -> bool {
    macro_rules! check {
        ($value:expr, $($size:expr),+) => {
            {
                let mut _it = $value;

                $(
                match _it.next() {
                    Some(part) if all_digits(part, $size) => {}
                    _ => return false,
                }
                )+

                if _it.next().is_some() {
                    return false;
                }
            }
        }
    }

    macro_rules! skip {
        ($value:expr) => {
            match $value.char_indices().skip(1).next() {
                Some((add, _)) => &$value[add..],
                None => return false,
            }
        }
    }

    let index = match input.find('T') {
        Some(index) => index,
        None => return false,
    };

    check!(input[..index].split('-'), 4, 2, 2);

    let mid = skip!(&input[index..]);

    let end = match mid.find(|c| c == '+' || c == 'Z') {
        Some(index) => index,
        None => return false,
    };

    check!(mid[..end].split(':'), 2, 2, 2);

    let tail = match mid[end..].chars().next() {
        Some('Z') => return true,
        Some('+') => skip!(&mid[end..]),
        _ => return false,
    };

    check!(tail.split(':'), 2, 2);
    return true;

    fn all_digits(input: &str, size: usize) -> bool {
        input.len() == size && input.chars().all(|c| c.is_digit(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(true, is_datetime("2018-02-01T00:02:02Z"));
        assert_eq!(true, is_datetime("2018-02-01T00:02:02+00:00"));
        assert_eq!(false, is_datetime("2018-02-01T00:02:02+00:00Z"));
        assert_eq!(false, is_datetime("-02-01T00:02:02+00:00Z"));
    }
}
