use thiserror::Error;

/// Represents the count of each color
#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl Color {
    pub fn new(record: &str) -> Result<Self, ColorParserError> {
        let (color, count) = parse_color_count(record)?;
        match color {
            "red" => Ok(Color::Red(count)),
            "green" => Ok(Color::Green(count)),
            "blue" => Ok(Color::Blue(count)),
            _ => Err(ColorParserError::InvalidColor(color.to_string())),
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ColorParserError {
    #[error("Invalid color count record: {0}")]
    InvalidColorCount(String),

    #[error("Invalid color: {0}")]
    InvalidColor(String),

    #[error("Invalid count: {0}")]
    InvalidCount(String),
}

/// Parses a color count record into a color and count
/// The record should be in the format: <count> <color>
/// For example: 1 red
fn parse_color_count(record: &str) -> Result<(&str, u8), ColorParserError> {
    let mut parts = record.trim().split(" ");
    let count_str = parts.next();
    let color_str = parts.next();

    // Extract the count and color parts
    if let (Some(count), Some(color)) = (count_str, color_str) {
        // Parse the count
        let count = count.parse::<u8>().or_else(|_| {
            Err(ColorParserError::InvalidCount(
                count_str
                    .unwrap_or_else(|| "Value not provided")
                    .to_string(),
            ))
        })?;
        Ok((color, count))
    } else {
        Err(ColorParserError::InvalidColorCount(record.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_red_color() {
        let record = "1 red";
        let color = Color::new(record).unwrap();
        assert_eq!(color, Color::Red(1));
    }

    #[test]
    fn creates_green_color() {
        let record = "2 green";
        let color = Color::new(record).unwrap();
        assert_eq!(color, Color::Green(2));
    }

    #[test]
    fn creates_blue_color() {
        let record = "3 blue";
        let color = Color::new(record).unwrap();
        assert_eq!(color, Color::Blue(3));
    }

    #[test]
    fn fails_to_create_color_with_invalid_color() {
        let record = "1 yellow";
        let result = Color::new(record);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidColor("yellow".to_string())
        );
    }

    #[test]
    fn fails_to_create_color_with_invalid_count() {
        let record = "five red";
        let result = Color::new(record);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidCount("five".to_string())
        );
    }

    #[test]
    fn test_parse_color_count() {
        let record = "1 red";
        let (color, count) = parse_color_count(record).unwrap();
        assert_eq!(color, "red");
        assert_eq!(count, 1);

        let record = "2 green";
        let (color, count) = parse_color_count(record).unwrap();
        assert_eq!(color, "green");
        assert_eq!(count, 2);

        let record = "3 blue";
        let (color, count) = parse_color_count(record).unwrap();
        assert_eq!(color, "blue");
        assert_eq!(count, 3);

        let record = "five red";
        let result = parse_color_count(record);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidCount("five".to_string())
        );
    }
}
