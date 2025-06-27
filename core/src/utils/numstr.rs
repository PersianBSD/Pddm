// core/src/utils/numstr.rs

/// Converts a number to string with optional base (default decimal)
pub fn to_string_with_base<T>(number: T, base: u32) -> String
where
    T: std::fmt::Display + std::fmt::LowerHex + std::fmt::Octal,
{
    match base {
        8 => format!("{:o}", number),  // Octal
        16 => format!("{:x}", number), // Hex
        _ => number.to_string(),       // Decimal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(to_string_with_base(42, 10), "42");
    }

    #[test]
    fn test_octal() {
        assert_eq!(to_string_with_base(42, 8), "52");
    }

    #[test]
    fn test_hex() {
        assert_eq!(to_string_with_base(42, 16), "2a");
    }
}
