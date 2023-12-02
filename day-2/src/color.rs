use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ColorParserError {
    #[error("Invalid color count record: {0}")]
    InvalidColorCount(String),

    #[error("Invalid color: {0}")]
    InvalidColor(String),

    #[error("Invalid count: {0}")]
    InvalidCount(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ColorCount {
    pub red: Option<u8>,
    pub green: Option<u8>,
    pub blue: Option<u8>,
}

impl ColorCount {
    pub fn default() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
        }
    }

    pub fn new(red: Option<u8>, green: Option<u8>, blue: Option<u8>) -> Self {
        Self { red, green, blue }
    }

    pub fn parse(color_counts_str: &str) -> Result<Self, ColorParserError> {
        parse_color_counts(color_counts_str)
    }
}

/// Parses colors into a color count
fn parse_color_counts(color_counts_str: &str) -> Result<ColorCount, ColorParserError> {
    let mut color_records = color_counts_str.split(",");
    let mut result = ColorCount::default();
    color_records.try_for_each(|record| match parse_color_count(record) {
        Ok((color, count)) => {
            match color {
                "red" => result.red = Some(count),
                "green" => result.green = Some(count),
                "blue" => result.blue = Some(count),
                _ => return Err(ColorParserError::InvalidColor(color.to_string())),
            }
            Ok(())
        }
        Err(e) => Err(e),
    })?;
    Ok(result)
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
                    .trim()
                    .to_string(),
            ))
        })?;
        Ok((color, count))
    } else {
        Err(ColorParserError::InvalidColorCount(
            record.trim().to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_a_valid_colors_string() {
        let colors_str = "1 red, 2 green, 3 blue";
        let color_count = parse_color_counts(colors_str).unwrap();
        assert_eq!(
            color_count,
            ColorCount {
                red: Some(1),
                green: Some(2),
                blue: Some(3)
            }
        );
    }

    #[test]
    fn fails_on_invalid_colors_string() {
        let colors_str = "1 red, 2 green, 3 yellow";
        let result = parse_color_counts(colors_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidColor("yellow".to_string())
        );

        let colors_str = "1 red, 2 green, 3";
        let result = parse_color_counts(colors_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ColorParserError::InvalidColorCount("3".to_string())
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
