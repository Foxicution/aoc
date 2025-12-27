pub trait Parse: Sized + Default {
    fn parse_next(cursor: &mut &[u8]) -> Option<Self>;
}

macro_rules! impl_unsigned {
    ($($t:ty),*) => {
        $(
            impl Parse for $t {
                #[inline(always)]
                fn parse_next(cursor: &mut &[u8]) -> Option<Self> {
                    while !cursor.is_empty() {
                        if cursor[0].is_ascii_digit() {
                            let mut acc: $t = 0;
                            while !cursor.is_empty() && cursor[0].is_ascii_digit() {
                                acc = acc * 10 + (cursor[0] - b'0') as $t;
                                *cursor = &cursor[1..];
                            }
                            return Some(acc);
                        }
                        *cursor = &cursor[1..];
                    }
                    None
                }
            }
        )*
    };
}

macro_rules! impl_signed {
    ($($t:ty),*) => {
        $(
            impl Parse for $t {
               #[inline(always)]
                fn parse_next(cursor: &mut &[u8]) -> Option<Self> {
                    while !cursor.is_empty() {
                        if cursor[0].is_ascii_digit() {
                            let mut acc: $t = 0;
                            while !cursor.is_empty() && cursor[0].is_ascii_digit() {
                                acc = acc * 10 + (cursor[0] - b'0') as $t;
                                *cursor = &cursor[1..];
                            }
                            return Some(acc);
                        }
                        else if cursor[0] == b'-' && cursor.len() > 1 && cursor[1].is_ascii_digit() {
                            *cursor = &cursor[1..];
                            let mut acc: $t = 0;
                            while !cursor.is_empty() && cursor[0].is_ascii_digit() {
                                acc = acc * 10 - (cursor[0] - b'0') as $t;
                                *cursor = &cursor[1..];
                            }
                            return Some(acc);
                        }

                        *cursor = &cursor[1..];
                    }
                    None
                }
             }
        )*
    };
}

impl_unsigned!(u8, u16, u32, u64, u128, usize);
impl_signed!(i8, i16, i32, i64, i128, isize);

pub fn numbers<T: Parse>(text: &str) -> impl Iterator<Item = T> + '_ {
    let mut cursor = text.as_bytes();
    std::iter::from_fn(move || T::parse_next(&mut cursor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("10, -10", vec![10, -10])]
    #[case::min_max("127, -128", vec![127, -128])]
    #[case::with_text("text-text -5", vec![-5])]
    #[case("12-12", vec![12, -12])]
    fn test_i8_parsing(#[case] input: &str, #[case] expected: Vec<i8>) {
        let result: Vec<i8> = numbers(input).collect();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("10 20 30", vec![10, 20, 30])]
    #[case("255", vec![255])]
    #[case::ignore_negatives("-5", vec![5])]
    #[case("12-12", vec![12, 12])]
    fn test_u8_parsing(#[case] input: &str, #[case] expected: Vec<u8>) {
        let result: Vec<u8> = numbers(input).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_i32_mixed_text() {
        let text = "Temp: -50C, Speed: 100m/s, Error: -999";
        let result: Vec<i32> = numbers(text).collect();
        assert_eq!(result, vec![-50, 100, -999]);
    }

    #[test]
    fn test_u32_ignores_negatives() {
        let text = "Values: -50, 100";
        let result: Vec<u32> = numbers(text).collect();
        assert_eq!(result, vec![50, 100]);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_u8_overflow_panics() {
        let text = "300";
        let _result: Vec<u8> = numbers(text).collect();
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_i8_underflow_panics() {
        let text = "-129";
        let _result: Vec<i8> = numbers(text).collect();
    }

    #[test]
    #[should_panic]
    fn test_i8_overflow_panics() {
        let text = "128";
        let _result: Vec<i8> = numbers(text).collect();
    }
}
