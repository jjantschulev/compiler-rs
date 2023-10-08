use std::str::Chars;

enum Base {
    Binary,
    Decimal,
    Hexadecimal,
}

impl Base {
    fn parse_from_str(c: &str) -> Self {
        if c.starts_with("0x") {
            Base::Hexadecimal
        } else if c.starts_with("0b") {
            Base::Binary
        } else {
            Base::Decimal
        }
    }

    fn prefix_len(&self) -> usize {
        match self {
            Base::Binary => 2,
            Base::Decimal => 0,
            Base::Hexadecimal => 2,
        }
    }

    fn radix(&self) -> u32 {
        match self {
            Base::Binary => 2,
            Base::Decimal => 10,
            Base::Hexadecimal => 16,
        }
    }
}

pub fn parse_int_literal(input: &str) -> Option<(i64, usize)> {
    let base = Base::parse_from_str(input);
    let mut len = base.prefix_len();
    let mut iterator = input.chars().skip(len);

    while iterator.next().is_some_and(|c| c.is_digit(base.radix())) {
        len += 1;
    }

    if (len - base.prefix_len()) == 0 {
        return None;
    }

    let number = i64::from_str_radix(&input[base.prefix_len()..len], base.radix()).ok()?;

    Some((number, len))
}

pub fn parse_float_literal(input: &str) -> Option<(f64, usize)> {
    let mut iterator = input.chars().peekable();
    let mut len_before = 0;

    while iterator.peek().is_some_and(|c| c.is_digit(10)) {
        iterator.next();
        len_before += 1;
    }

    if iterator.next() != Some('.') {
        return None;
    }

    let mut len_after = len_before + 1;

    while iterator.next().is_some_and(|c| c.is_digit(10)) {
        len_after += 1;
    }

    if len_after == len_before + 1 {
        return None;
    }

    let number = input[..len_after].parse::<f64>().ok()?;
    Some((number, len_after))
}

fn parse_next_char(iterator: &mut Chars) -> Option<(char, usize)> {
    let (c, len) = match iterator.next() {
        Some('\\') => match iterator.next() {
            Some('n') => ('\n', 2),
            Some('r') => ('\r', 2),
            Some('t') => ('\t', 2),
            Some('\'') => ('\'', 2),
            Some('\\') => ('\\', 2),
            Some('0') => ('\0', 2),
            Some('x') => match (iterator.next(), iterator.next()) {
                (Some(a), Some(b)) => {
                    let a = a.to_digit(16)?;
                    let b = b.to_digit(16)?;
                    let c = (a << 4) | b;
                    (c as u8 as char, 4)
                }
                _ => return None,
            },
            _ => return None,
        },
        Some(c) => (c, 1),
        _ => return None,
    };

    Some((c, len))
}

pub fn parse_char_literal(input: &str) -> Option<(char, usize)> {
    let mut iterator = input.chars();
    if iterator.next() != Some('\'') {
        return None;
    }

    let (c, len) = parse_next_char(&mut iterator)?;

    if iterator.next() != Some('\'') {
        return None;
    }

    Some((c, len + 2))
}

pub fn parse_string_literal(input: &str) -> Option<(String, usize)> {
    let mut iterator = input.chars();
    if iterator.next() != Some('"') {
        return None;
    }

    let mut len = 1;
    let mut string = String::new();

    loop {
        match parse_next_char(&mut iterator)? {
            ('"', 1) => return Some((string, len + 1)),
            (c, l) => {
                len += l;
                string.push(c);
            }
        };
    }
}
